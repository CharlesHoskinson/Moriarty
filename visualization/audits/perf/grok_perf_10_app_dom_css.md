# Perf Audit 10 — App Controller, DOM Updates, Layout & CSS Compositing

Auditor: Grok 4.7 xhigh Performance Auditor #10 (App Controller, DOM Updates, Layout & CSS Compositing)
Date: 2026-10-18
Scope: `src/app.js` (977 lines), `index.html` (566 lines), `src/styles.css` (2042 lines)
Method: source inspection of the step-playback and scrub paths, the module graph those files pull in, and the CSS that paints around them. No source files were modified. Costs below are structural (what the browser must do), not a captured profile.

Constraint used for every recommendation: the settled pixels, motion, keyboard map, accessible names, and click targets stay the same. Where today's `innerHTML` replacement accidentally skips a CSS transition, the replacement keeps that snap.

## Verdict

Startup waits on a deep module waterfall, and every integer step rebuilds the inspector.

`els` already caches `getElementById` (`src/app.js` 56–97). `animate` already calls `syncStepUI` only when the integer step changes (`src/app.js` 965–967). Telemetry, rings, frames, and calls already compare a previous HTML string before writing (`src/app.js` 545–548, 563–566, 575–578, 582–585, 589–592, 685–688, 727–730, 768–771). Those guards are not the hot cost.

On a step that changes anything visible, `syncStepUI` still:

1. Allocates the full rings / frames / calls / telemetry HTML on every call, then compares it.
2. Parses that HTML and destroys the previous nodes whenever the string differs. The step index is inside the transition string, so `#telemetryTransition` is rebuilt on every step. The frontier class, fuel, and control sector are inside the rings string, so `#sixRingsGauge` (about 80 nodes) is rebuilt on almost every step.
3. Assigns `textContent` and `className` for the caption, scrubber, badges, and mode pill even when the string did not change (`src/app.js` 474–541, 744–753).
4. Re-runs an `entry` regex and `getQuinelingTelemetry()` on every step (`src/app.js` 501–517, 555–567).
5. Does this synchronously inside the scrubber `input` handler (`src/app.js` 857–865), which can fire many times per frame. `animate` would have published the same integer step once, on the next frame, before paint.

The CSS file has no `contain`, no `content-visibility`, and no compositor isolation around the canvas. `.ring-pill` and `.rec-call-node` declare color transitions that never run today, because `innerHTML` creates a new node already in the final state. Pooling the nodes without turning those transitions off would add a fade that is not on screen now.

`index.html` discovers `app.js` only at the bottom of the body (line 564). `three.webgpu.js` (2.2 MB) and its import `three.core.js` (1.4 MB) are two and three hops away. Nothing `modulepreload`s them. `three/tsl` is in the import map and is not imported by the live graph.

## What is already doing the right work

Keep these. The replacements below sit on top of them.

| Guard | Where | What it actually saves |
| --- | --- | --- |
| `els` id cache | `src/app.js` 56–97 | No repeat `getElementById` on the hot path. The script is at the end of `<body>`, so the nodes exist when the module evaluates. |
| Integer step gate | `src/app.js` 30, 390, 459, 965–967 | Fractional playhead motion does not touch the DOM. `stage.render` still runs every frame, which is the 3D auditor's problem. |
| HTML equality cache | `src/app.js` 31–40 and the `last*Html` checks | Skips the parse when a panel is byte-identical. Does not skip building the string, and does not skip the parse on a normal step. |
| Line-number early return | `src/app.js` 346–350 | Skips the gutter rebuild when the line count and active line are unchanged. |
| Step-state cache | `src/app.js` 910–942 | `buildStepState` runs once per integer step, then the animation frame only writes `stepFraction`, `dt`, and `ambient` onto the cached object. |
| One delegated click on the call track | `src/app.js` 780–790 | Call chips do not each own a listener. Pooling must keep this. Do not bind a listener per chip. |
| `requestAnimationFrame` loop | `src/app.js` 946–977 | Already the right publisher for playhead → DOM. Transport handlers duplicate it. |
| Font `preconnect` + `display=swap` | `index.html` 8–10 | Correct. Do not switch this stylesheet to `media="print"` / async. That adds a fallback flash the current page does not have. |

## Hot path, as it runs today

`animate` (`src/app.js` 946–970) advances `state.playhead` by `dt * 3.2 * state.speed`. At 1× that is a new integer step roughly every 300 ms. At 4× it is roughly every 80 ms. Each new integer calls `syncStepUI` (455–773), which always does all of the following before any equality check:

- Copies and reverses the frame array (`521`).
- Formats control text, status, and the equation (`464–466`, `269–283`, `126–136`).
- Walks `programImage.expressions` and runs `/\bentry\s+([\s\S]*?)\s*;/` (`506–511`).
- Calls `getQuinelingTelemetry` (`555–560`), which calls `resolveCompiledQuineling` → `compileSherlockProgramToQuineling`. The compiler memoizes by source (`src/engine/sherlockQuinelingCompiler.js` 604–607), but the wrapper still allocates a fresh telemetry object and the DOM path still builds an HTML string.
- Builds six ring cards, twelve slot rows, and ten sector chips as one HTML string (`606–683`), then maybe assigns `innerHTML` (`687`).
- Builds every saved-frame card (`693–726`) and every call chip (`755–767`).
- Writes the scrubber value and `aria-valuetext` (`487–493`).

Scrubbing (`857–865`) calls that function from the `input` event itself. A drag across the trace can cross every integer step in a few frames. Each event parses a new inspector and lays it out. The next `animate` then sees `lastRenderedStepIdx` already equal and skips. The synchronous call is extra work the user cannot see, because paint has not happened yet.

Node count destroyed on a typical step when the strings differ:

| Write | Lines | Nodes parsed |
| --- | --- | --- |
| `#sixRingsGauge` | 626–687 | ~12 per ring × 6, plus the stator strip and 10 chips. About 80–90. |
| `#frameStackList` | 693–729 | 4 per frame, or one empty paragraph. |
| `#recursionCallsTrack` | 755–770 | One button per call, plus an arrow span between them. |
| `#telemetryTransition` | 544–548 | Text plus `<small>`, every step, because the index changed. |
| `#telemetryQdlMorph`, `#telemetryFrames`, `#telemetryCalls`, `#telemetryFuel` | 562–592 | Each is an element replace whenever that one string changes. Fuel and the rule line change on most charged steps. |
| `#editorLineNumbers` | 346–361 | One text node or `<span>` per source line, whenever the active line changes. |

`innerHTML` also drops focus if it was inside the call track, and it resets `#editorLineNumbers` `scrollTop` when the gutter is rebuilt. Both are side effects of the rebuild, not designed behavior.

## P0 — Stop rebuilding the inspector on each step

Land the DOM pools and the transition removal in one change. Pooling alone makes `.ring-pill` and `.rec-call-node` honor transitions that `innerHTML` currently bypasses.

### P0.1 Persistent ring gauge and control stator

**Where:** `src/app.js` 595–689, fed by `getFrameSlotMeta` (149–267), `resolveChronoEnvHint` (427–453), and `CONTROL_STRIP_SECTORS` (43–54). The host node is the empty `<div id="sixRingsGauge">` at `index.html` 229.

**Why the string cache loses:** `ringsHtml` includes the frontier class, the needle hemisphere, `fuelPct`, and the active sector. Those move on a normal step, so `ringsHtml !== state.lastRingsHtml` is true and line 687 replaces the whole gauge. The static parts of that template never change: ring ids `R1 (02)` … `R6 (12)`, slot index labels `01–02` … `11–12`, the words `West A:` / `East B:`, the ten badges, and the stator heading.

**Pixel contract:** same three-column grid (`.ring-pill` is `64px 1fr 1fr` at `src/styles.css` 1280–1292), same slot class strings (`slot-idle`, `slot-env slot-env-return`, `slot-op slot-op-right`, `slot-frontier`, `slot-overflow`, …), same `title` strings, same `aria-current="true"` only on the frontier slot, same overflow badge text `+N overflow` only when `r === 5 && overflowFrames > 0`, same sector chip classes (`active`, `is-done`, `is-fault`) and `aria-current="step"` only on the active sector.

Ring ids are constant. Compute them once:

```javascript
function mountRingsGauge(host) {
  const rings = [];
  for (let r = 0; r < 6; r++) {
    const slotLo = String(r * 2 + 1).padStart(2, '0');
    const slotHi = String(r * 2 + 2).padStart(2, '0');
    const pill = document.createElement('div');
    pill.className = 'ring-pill ring-idle';

    const top = document.createElement('div');
    top.className = 'ring-pill-top';
    const idEl = document.createElement('span');
    idEl.className = 'ring-id';
    idEl.textContent = `R${r + 1} (${slotHi})`;
    const idxEl = document.createElement('span');
    idxEl.className = 'ring-slots-idx';
    idxEl.textContent = `${slotLo}–${slotHi}`;
    top.append(idEl, idxEl);

    const makeSlot = (keyText) => {
      const row = document.createElement('div');
      row.className = 'ring-slot-row slot-idle';
      const main = document.createElement('span');
      main.className = 'slot-main';
      const key = document.createElement('span');
      key.className = 'slot-key';
      key.textContent = keyText;
      const val = document.createElement('span');
      val.className = 'slot-val';
      val.textContent = '—';
      main.append(key, val);
      row.append(main);
      return { row, val };
    };

    const slotA = makeSlot('West A:');
    const slotB = makeSlot('East B:');
    const badge = document.createElement('span');
    badge.className = 'overflow-badge';
    badge.hidden = true;
    slotB.row.append(badge);
    pill.append(top, slotA.row, slotB.row);
    host.append(pill);
    rings.push({
      pill, slotA, slotB, badge,
      lastPillClass: pill.className,
      lastAClass: slotA.row.className,
      lastBClass: slotB.row.className,
    });
  }

  const strip = document.createElement('div');
  strip.className = 'control-sector-strip';
  strip.setAttribute('aria-label', '10-Sector Control Stator');
  const stripTop = document.createElement('div');
  stripTop.className = 'control-strip-top';
  const stripLabel = document.createElement('span');
  stripLabel.textContent = '10-Sector Control Stator';
  const needle = document.createElement('strong');
  stripTop.append(stripLabel, needle);
  const grid = document.createElement('div');
  grid.className = 'control-sector-grid';
  const sectors = CONTROL_STRIP_SECTORS.map((sec) => {
    const chip = document.createElement('span');
    chip.className = 'control-sector-chip';
    chip.textContent = sec.badge;
    chip.title = sec.title;
    grid.append(chip);
    return chip;
  });
  strip.append(stripTop, grid);
  host.append(strip);
  return { rings, needle, sectors };
}
```

Call `const ringsView = mountRingsGauge(els.sixRingsGauge)` once, next to the other init at `src/app.js` 973–977, before `executeCurrentSource(true)`.

Replace the body of the block at `src/app.js` 595–689 (keep the `ringsSummaryBadge` text write, but run it through `setText` from P1.1) with a patch. `textContent` and `element.title` must not be HTML-escaped. `escapeHtml` was only there because the strings were interpolated into HTML. Writing the raw readout produces the same glyphs.

```javascript
function setClass(el, next, slot, key) {
  if (slot[key] === next) return;
  slot[key] = next;
  el.className = next;
}

function setText(el, next, slot, key) {
  const value = next == null ? '' : String(next);
  if (slot[key] === value) return;
  slot[key] = value;
  el.textContent = value;
}

function setTitle(el, next, slot, key) {
  if (slot[key] === next) return;
  slot[key] = next;
  el.title = next;
}

function setCurrent(el, on, value) {
  if (on) {
    if (el.getAttribute('aria-current') !== value) el.setAttribute('aria-current', value);
  } else if (el.hasAttribute('aria-current')) {
    el.removeAttribute('aria-current');
  }
}

// inside syncStepUI, in place of the ringsHtml loop:
const frontierChronoIdx = chronoLen > 0 ? Math.min(11, chronoLen - 1) : -1;
const activeRingIdx = frontierChronoIdx >= 0 ? Math.floor(frontierChronoIdx / 2) : -1;

for (let r = 0; r < 6; r++) {
  const view = ringsView.rings[r];
  const idxA = r * 2;
  const idxB = r * 2 + 1;
  const fA = frameAtChrono(frames, idxA);
  const fB = frameAtChrono(frames, idxB);
  const fill = (fA ? 1 : 0) + (fB ? 1 : 0);
  const metaA = getFrameSlotMeta(fA, resolveChronoEnvHint(frames, idxA, activeBindings));
  const metaB = getFrameSlotMeta(fB, resolveChronoEnvHint(frames, idxB, activeBindings));
  const isFocus = r === activeRingIdx;
  const isFrontierA = idxA === frontierChronoIdx;
  const isFrontierB = idxB === frontierChronoIdx;
  const isOverflow = r === 5 && overflowFrames > 0;
  const ringState = fill === 0 ? 'ring-idle' : isFocus ? 'ring-active' : 'ring-occupied';
  const slotLo = String(idxA + 1).padStart(2, '0');
  const slotHi = String(idxB + 1).padStart(2, '0');

  setClass(view.pill, `ring-pill ${ringState}`, view, 'lastPillClass');
  setTitle(
    view.pill,
    `Ring R${r + 1} (Chrono Slots ${slotLo}–${slotHi}) · West A: ${metaA.readout} | East B: ${metaB.readout}`,
    view,
    'lastPillTitle'
  );

  setClass(
    view.slotA.row,
    `ring-slot-row ${metaA.cls}${isFrontierA ? ' slot-frontier' : ''}`,
    view,
    'lastAClass'
  );
  setText(view.slotA.val, metaA.readout, view, 'lastAText');
  setTitle(view.slotA.row, `West Slot A (#${slotLo}): ${metaA.tooltip}`, view, 'lastATitle');
  setCurrent(view.slotA.row, isFrontierA, 'true');

  setClass(
    view.slotB.row,
    `ring-slot-row ${metaB.cls}${isFrontierB ? ' slot-frontier' : ''}${isOverflow ? ' slot-overflow' : ''}`,
    view,
    'lastBClass'
  );
  setText(view.slotB.val, metaB.readout, view, 'lastBText');
  setTitle(
    view.slotB.row,
    `East Slot B (#${slotHi}): ${metaB.tooltip}${isOverflow ? ` (+${overflowFrames} overflow beyond 12-slot window)` : ''}`,
    view,
    'lastBTitle'
  );
  setCurrent(view.slotB.row, isFrontierB, 'true');

  if (view.badge.hidden !== !isOverflow) view.badge.hidden = !isOverflow;
  if (isOverflow) {
    const badgeText = `+${overflowFrames} overflow`;
    setText(view.badge, badgeText, view, 'lastBadge');
    setTitle(view.badge, `${overflowFrames} additional frame(s) beyond the 12-slot window`, view, 'lastBadgeTitle');
  }
}

const activeSectorKey =
  statusInfo.label === 'Done'
    ? 'Return'
    : statusInfo.label === 'Rejected' || statusInfo.label === 'Exhausted'
      ? 'Error'
      : rawCtrlKind;
const needleText =
  frontierChronoIdx < 0
    ? 'Needle → Center (0 frames)'
    : frontierChronoIdx % 2 === 0
      ? `Needle → West Slot A (R${activeRingIdx + 1}·A)`
      : `Needle → East Slot B (R${activeRingIdx + 1}·B)`;
setText(ringsView.needle, needleText, ringsView, 'lastNeedle');

for (let i = 0; i < CONTROL_STRIP_SECTORS.length; i++) {
  const sec = CONTROL_STRIP_SECTORS[i];
  const chip = ringsView.sectors[i];
  const isOn = sec.key.toLowerCase() === activeSectorKey.toLowerCase();
  const tone = !isOn
    ? ''
    : statusInfo.label === 'Done'
      ? ' is-done'
      : statusInfo.label === 'Rejected' || statusInfo.label === 'Exhausted' || sec.key === 'Error'
        ? ' is-fault'
        : '';
  const cls = `control-sector-chip${isOn ? ' active' : ''}${tone}`;
  if (chip.className !== cls) chip.className = cls;
  setCurrent(chip, isOn, 'step');
}
```

`frameAtChrono` replaces `const chronoFrames = [...frames].reverse()` at line 521. Newest frame is `frames[0]`, and chronological slot 0 is the oldest, which is what `.reverse()` produces today:

```javascript
function frameAtChrono(frames, chronoIdx) {
  return frames[frames.length - 1 - chronoIdx];
}
```

Change `resolveChronoEnvHint` (`427–453`) to take `frames` plus a chrono index, and to read `frameAtChrono(frames, j)` instead of `chronoFrames[j]`. Same walk, no copied array. Precomputing the twelve hints once per step is the same function called in order; do that if the backward walk shows up in a profile. It is not required for pixel identity.

Delete `state.lastRingsHtml` (line 31) once this lands. The retained string is a second copy of the gauge.

`.overflow-badge` is `display: inline-flex` (`src/styles.css` 1451–1460). In browsers where the author `display` beats the UA `[hidden]` rule, the attribute alone will not hide the badge. Add the author rule in P2.4 in the same change.

### P0.2 Saved-frame list as a pool

**Where:** `src/app.js` 691–730. Host: `<div class="frames-list" id="frameStackList">` at `index.html` 238. CSS: `src/styles.css` 1524–1602.

The frame stack is stable across many steps (an `Eval` that does not push). The current code still allocates `framesHtml` and walks every frame through `getFrameSlotMeta`, `formatAlgebraicText`, and `resolveFrameEnvText` before the equality check at 727.

Build the empty note and grow the pool only when `frames.length` exceeds it. Slider max is 20 (`index.html` 110), but the pool must grow from the trace, not from that attribute.

```javascript
function mountFrameList(host) {
  const empty = document.createElement('p');
  empty.className = 'empty-note';
  empty.hidden = true;
  host.append(empty);
  return { host, empty, items: [] };
}

function ensureFrameItem(pool) {
  const root = document.createElement('div');
  root.className = 'frame-item';
  const top = document.createElement('div');
  top.className = 'frame-item-top';
  const op = document.createElement('span');
  op.className = 'frame-op';
  const tag = document.createElement('span');
  tag.className = 'frame-kind-tag';
  const env = document.createElement('div');
  env.className = 'frame-env-row';
  top.append(op, tag);
  root.append(top, env);
  pool.host.append(root);
  const item = { root, op, tag, env, hidden: false };
  pool.items.push(item);
  return item;
}

function patchFrameList(pool, frames, activeBindings, stackLimit) {
  setText(els.frameStackCount, `${frames.length} of ${stackLimit} limit`, pool, 'lastCount');

  if (frames.length === 0) {
    const rootEnvStr =
      activeBindings.length > 0
        ? `Active env: ${activeBindings.map((b) => `${b.name} = ${b.valueSummary}`).join(', ')}`
        : 'No saved frames.';
    setText(pool.empty, rootEnvStr, pool, 'lastEmpty');
    if (pool.empty.hidden) pool.empty.hidden = false;
    for (const item of pool.items) {
      if (!item.hidden) {
        item.hidden = true;
        item.root.hidden = true;
      }
    }
    return;
  }

  if (!pool.empty.hidden) pool.empty.hidden = true;
  while (pool.items.length < frames.length) ensureFrameItem(pool);

  for (let idx = 0; idx < pool.items.length; idx++) {
    const item = pool.items[idx];
    const fr = frames[idx];
    if (!fr) {
      if (!item.hidden) {
        item.hidden = true;
        item.root.hidden = true;
      }
      continue;
    }
    if (item.hidden) {
      item.hidden = false;
      item.root.hidden = false;
    }
    const slotMeta = getFrameSlotMeta(fr);
    const chronoIdx = frames.length - 1 - idx;
    const ringNum = Math.floor(chronoIdx / 2) + 1;
    const slotChar = chronoIdx % 2 === 0 ? 'A' : 'B';
    const topClass = idx === 0 ? ' is-top' : '';
    const cls = `frame-item ${slotMeta.itemCls}${topClass}`;
    if (item.root.className !== cls) item.root.className = cls;
    setText(item.op, formatAlgebraicText(fr.label), item, 'lastOp');
    const kindTag =
      ringNum <= 6
        ? `R${ringNum}·${slotChar} · ${slotMeta.family} · ${fr.kind}`
        : `+${chronoIdx - 11} overflow · ${slotMeta.family} · ${fr.kind}`;
    setText(item.tag, kindTag, item, 'lastTag');
    const borrowed = fr.kind === 'Right' || fr.kind === 'Unary' ? ' (enclosing)' : '';
    setText(
      item.env,
      `Env${borrowed}: ${resolveFrameEnvText(frames, idx, activeBindings)}`,
      item,
      'lastEnv'
    );
  }
}
```

`hidden` on a `.frame-item` removes it from the flex layout, so the 8px gap (`src/styles.css` 1524–1529) matches a list that never contained the extra rows. Delete `state.lastFramesHtml` (line 32).

A cheap signature in front of `patchFrameList` preserves the same output and skips the walk when the stack did not change:

```javascript
function frameStackSignature(frames, activeBindings, stackLimit) {
  let sig = `${frames.length}|${stackLimit}|${activeBindings.length}`;
  for (let i = 0; i < frames.length; i++) {
    const fr = frames[i];
    const n = fr.bindings ? fr.bindings.length : 0;
    sig += `|${fr.kind}\u0001${fr.label}\u0001${n}`;
  }
  return sig;
}
```

`resolveFrameEnvText` also depends on binding names and values, and on borrowed enclosing bindings. Extend the signature with those if a trace rewrites a binding in place without changing `kind` / `label` / count. If `StepRecord` frames are fresh objects per step but the fields above are equal, the signature still matches and the DOM stays still. That is the same picture `lastFramesHtml` was trying to keep.

### P0.3 Active-call chips as a pool

**Where:** `src/app.js` 732–772. Host: `#recursionCallsTrack` at `index.html` 247. Listener stays at `src/app.js` 780–790.

Same rule as the rings: a new button created with the final class does not animate. Reused buttons would. P2.3 turns the transition off so a status change still snaps from `.waiting` to `.active` to `.returned`.

```javascript
function mountCallTrack(host) {
  const empty = document.createElement('span');
  empty.className = 'empty-note';
  empty.hidden = true;
  host.append(empty);
  return { host, empty, nodes: [], arrows: [] };
}

function patchCallTrack(pool, rec, controlText) {
  // mode pill is updated separately, through setText / setClass (P1.1)
  if (!rec || rec.calls.length === 0) {
    setText(pool.empty, `Entry expression → ${controlText}`, pool, 'lastEmpty');
    if (pool.empty.hidden) pool.empty.hidden = false;
    for (const node of pool.nodes) if (!node.hidden) { node.hidden = true; node.btn.hidden = true; }
    for (const arrow of pool.arrows) if (!arrow.hidden) { arrow.hidden = true; arrow.el.hidden = true; }
    return;
  }
  if (!pool.empty.hidden) pool.empty.hidden = true;

  const calls = rec.calls;
  while (pool.nodes.length < calls.length) {
    const btn = document.createElement('button');
    btn.type = 'button';
    btn.className = 'rec-call-node';
    pool.host.append(btn);
    pool.nodes.push({ btn, hidden: false });
    const arrow = document.createElement('span');
    arrow.className = 'rec-call-arrow';
    arrow.textContent = '→';
    arrow.hidden = true;
    pool.host.append(arrow);
    pool.arrows.push({ el: arrow, hidden: true });
  }

  for (let i = 0; i < pool.nodes.length; i++) {
    const node = pool.nodes[i];
    const arrow = pool.arrows[i];
    const call = calls[i];
    if (!call) {
      if (!node.hidden) { node.hidden = true; node.btn.hidden = true; }
      if (!arrow.hidden) { arrow.hidden = true; arrow.el.hidden = true; }
      continue;
    }
    if (node.hidden) { node.hidden = false; node.btn.hidden = false; }
    const cls = `rec-call-node ${call.status}`;
    if (node.btn.className !== cls) node.btn.className = cls;
    const retSuffix = call.status === 'returned' && call.returnValue ? ` = ${call.returnValue}` : '';
    setText(node.btn, `${call.label}${retSuffix}`, node, 'lastLabel');
    const stepTarget = call.status === 'returned' && call.returnStep !== null ? call.returnStep : call.invokeStep;
    const stepAttr = String(stepTarget);
    if (node.btn.dataset.step !== stepAttr) node.btn.dataset.step = stepAttr;
    const title = `Jump to ${call.label}`;
    if (node.btn.title !== title) node.btn.title = title;
    const showArrow = i < calls.length - 1;
    if (arrow.hidden !== !showArrow) {
      arrow.hidden = !showArrow;
      arrow.el.hidden = !showArrow;
    }
  }
}
```

`ev.target.closest('.rec-call-node')` keeps working. `dataset.step` stays the jump target. Delete `state.lastCallsHtml` (line 33).

Focus: `innerHTML` drops a focused chip every step. The pool keeps the same button element for call index `i`, so keyboard focus stays on that chip while its label updates. The label still shows the same string the old HTML showed. That is the interaction the chip was built for.

### P0.4 Telemetry leaves, not `innerHTML`

**Where:** `src/app.js` 534–593 and the `<dl class="telemetry-dl">` at `index.html` 187–216.

`#telemetryTransition` changes on every step, so the string cache never helps. The other four help only when that one field is stable, and they still allocate the string first. Splitting each `<dd>` into a text node plus a `<small>` makes a step change two `textContent` writes. The `<small>` keeps `src/styles.css` 1179–1183 (weight 400, muted, 11px).

Replace `index.html` 189–215 with the same visible placeholders, split so the middle dot lives in the `<small>`, which is what `&middot;` becomes after HTML parse:

```html
<dl class="telemetry-dl">
  <div>
    <dt>Status</dt>
    <dd id="telemetryStatus">Evaluating</dd>
  </div>
  <div>
    <dt>Transition</dt>
    <dd id="telemetryTransition"><span id="telemetryTransitionMain">1 / 45</span> <small id="telemetryTransitionSub">Rule A102</small></dd>
  </div>
  <div>
    <dt>Current control</dt>
    <dd class="mono" id="telemetryControl">Eval(fact(4))</dd>
  </div>
  <div>
    <dt>Quineling Core</dt>
    <dd id="telemetryQdlMorph"><strong id="telemetryQdlName">Nautilus Arithmeticus</strong> <small id="telemetryQdlSub">&middot; logarithmic-mantle</small></dd>
  </div>
  <div>
    <dt>Saved frames</dt>
    <dd id="telemetryFrames"><span id="telemetryFramesMain">0</span> <small id="telemetryFramesSub">&middot; 0 of 6 rings</small></dd>
  </div>
  <div>
    <dt>Active calls</dt>
    <dd id="telemetryCalls"><span id="telemetryCallsMain">0</span> <small id="telemetryCallsSub">&middot; 0 total</small></dd>
  </div>
  <div>
    <dt>Remaining fuel</dt>
    <dd id="telemetryFuel"><span id="telemetryFuelMain">200 / 200</span> <small id="telemetryFuelSub">&middot; charge 1</small></dd>
  </div>
</dl>
```

The pre-JS calls cell currently says `0` (`index.html` 211) and the first `syncStepUI` rewrites it to `0 <small>· N total</small>`. The skeleton matches the post-sync picture, which is what stays on screen. The space between `</span>` and `<small>` is the same space the old template had between the number and `<small>`.

Cache the new nodes in `els` (`src/app.js` 82–89):

```javascript
telemetryTransitionMain: document.getElementById('telemetryTransitionMain'),
telemetryTransitionSub: document.getElementById('telemetryTransitionSub'),
telemetryQdlName: document.getElementById('telemetryQdlName'),
telemetryQdlSub: document.getElementById('telemetryQdlSub'),
telemetryFramesMain: document.getElementById('telemetryFramesMain'),
telemetryFramesSub: document.getElementById('telemetryFramesSub'),
telemetryCallsMain: document.getElementById('telemetryCallsMain'),
telemetryCallsSub: document.getElementById('telemetryCallsSub'),
telemetryFuelMain: document.getElementById('telemetryFuelMain'),
telemetryFuelSub: document.getElementById('telemetryFuelSub'),
```

Replace the five `innerHTML` blocks with:

```javascript
setText(els.telemetryRuleBadge, `Rule ${step.ruleId}`, domMemo, 'rule');
setText(
  els.telemetryStatus,
  statusInfo.label === 'Done' && state.trace.outcome?.summary
    ? `Done (${state.trace.outcome.summary})`
    : statusInfo.label,
  domMemo,
  'status'
);
setClass(els.telemetryStatus, statusInfo.ddClass, domMemo, 'statusClass');

setText(els.telemetryTransitionMain, `${clampedIdx + 1} / ${totalSteps}`, domMemo, 'trMain');
setText(els.telemetryTransitionSub, `Rule ${step.ruleId} · ${step.ruleName}`, domMemo, 'trSub');
setText(els.telemetryControl, controlText, domMemo, 'ctrl');

setText(els.telemetryQdlName, qdlTel.speciesName, domMemo, 'qdlName');
setText(
  els.telemetryQdlSub,
  `· ${qdlTel.family} (${qdlTel.mechanism}) · ${qdlTel.morphGesture}`,
  domMemo,
  'qdlSub'
);

const ringNote =
  overflowFrames > 0
    ? `${Math.min(12, chronoLen)}/12 slots (${litRings}/6 rings, +${overflowFrames} overflow)`
    : `${chronoLen}/12 slots (${litRings}/6 rings)`;
setText(els.telemetryFramesMain, String(frames.length), domMemo, 'frMain');
setText(els.telemetryFramesSub, `· ${ringNote}`, domMemo, 'frSub');
setText(els.telemetryCallsMain, String(activeCallCount), domMemo, 'callMain');
setText(
  els.telemetryCallsSub,
  `· ${rec ? rec.calls.length : 0} total`,
  domMemo,
  'callSub'
);
setText(els.telemetryFuelMain, `${step.after.fuel} / ${initialFuel}`, domMemo, 'fuelMain');
setText(els.telemetryFuelSub, `· charge ${step.charge}`, domMemo, 'fuelSub');
```

`setClass` on `#telemetryStatus` must assign only `statusInfo.ddClass` (`is-evaluating`, `is-done`, `is-stopped`). That is what line 541 assigns today. Do not touch `class="mono"` on `#telemetryControl`.

Delete `lastTransitionHtml`, `lastQdlMorphHtml`, `lastFramesTelemetryHtml`, `lastCallsTelemetryHtml`, and `lastFuelTelemetryHtml` (`src/app.js` 36–40). `domMemo` is a plain object that holds the last written string per key.

`#telemetryCalls` initial small is the only placeholder that gains text versus today's raw HTML. After the first `syncStepUI`, which already runs from `executeCurrentSource` before the first paint the user can read, the strings match the old template.

### P0.5 Let `animate` publish scrub and key-repeat steps

**Where:** `src/app.js` 830–865 and 946–970.

`animate` already reads `state.playhead`, floors it, and calls `syncStepUI` when the integer changes. The scrubber, step buttons, and arrow-key `click()` handlers do the same work again inside the event, before the frame paints. A range `input` can cross several integers in one frame. Each one lays out the inspector. The paint shows only the last one.

Keep the playhead write in the event. Drop the synchronous `syncStepUI` from the high-rate handlers. The frame's existing rAF reads the latest playhead and paints it once.

```javascript
function publishPlayhead(next, { pause = true } = {}) {
  if (pause && state.playing) {
    state.playing = false;
    updatePlayButton();
  }
  const maxStep = state.trace ? state.trace.steps.length - 1 : 0;
  state.playhead = Math.max(0, Math.min(maxStep, next));
  // animate() floors this and calls syncStepUI once per painted frame.
}

els.stepPrevBtn.addEventListener('click', () => {
  publishPlayhead(Math.floor(state.playhead) - 1);
});

els.stepNextBtn.addEventListener('click', () => {
  publishPlayhead(Math.floor(state.playhead) + 1);
});

if (els.resetStepBtn) {
  els.resetStepBtn.addEventListener('click', () => {
    publishPlayhead(0);
  });
}

els.stepScrubber.addEventListener('input', () => {
  publishPlayhead(Math.round(Number(els.stepScrubber.value)));
});
```

`executeCurrentSource` (`390–391`) should still call `syncStepUI` itself. The compile finishes in a task, and the inspector should be current before the following paint even if that task runs after this frame's `animate`. It already sets `lastRenderedStepIdx = -1` first, so the following `animate` does not do the work twice.

Do not remove the `playing` check inside `animate`. While paused, `playhead` stays put and `stage.render` still runs. The DOM gate at 965 remains the right gate.

While a scrub is in progress, `syncStepUI` must not write `stepScrubber.value` if it already equals the integer. Assigning `.value` during the drag lays out the control again and can move the thumb. Playback still needs the write, because the thumb has to follow the clock:

```javascript
const scrubValue = String(clampedIdx);
if (els.stepScrubber.value !== scrubValue) els.stepScrubber.value = scrubValue;
const valueText = `Step ${clampedIdx + 1} of ${totalSteps}, ${statusInfo.label}, Rule ${step.ruleId}`;
if (els.stepScrubber.getAttribute('aria-valuetext') !== valueText) {
  els.stepScrubber.setAttribute('aria-valuetext', valueText);
}
```

That replaces `src/app.js` 487–493.

## P1 — Dirty checks, query caches, and work that does not change the picture

### P1.1 One write helper for the caption, transport, and mode pill

**Where:** `src/app.js` 469–541, 692, 744–753, 775–777.

`#stageStatusBadge` already skips an identical `textContent` (471–473) and then always assigns `className` (474). Assigning the same class string still tokenizes the attribute and restyles the badge. Status stays `status-evaluating` for almost the whole trace. The same pattern is on `#telemetryStatus` (537–541) and `#recursionModePill` (744–753).

These writes change on some steps and not on others. Guard all of them with the helpers from P0.1. `className` strings must stay exactly the strings written today.

| Element | Lines | Replacement |
| --- | --- | --- |
| `#stageStatusBadge` | 470–475 | `setText(..., statusInfo.label)` and `setClass(..., \`status-badge ${statusInfo.badgeClass}\`)` |
| `#hudControlSummary` | 476–478 | `setText(..., controlText)` |
| `#recursionEquationText` | 479–481 | `setText(..., equationText)` |
| `#scrubberStepLabel` | 484–486 | `setText(..., \`Step ${clampedIdx + 1} / ${totalSteps}\`)` |
| `#stepPrevBtn` / `#stepNextBtn` | 494–499 | `if (btn.disabled !== next) btn.disabled = next` with `next = clampedIdx <= 0` and `next = clampedIdx >= totalSteps - 1` |
| `#activeSpanBadge` | 514–516 | `setText(..., \`Line ${origin.line}\`)` |
| `#ringsSummaryBadge` | 598–601 | `setText` of the same two template strings |
| `#recursionModePill` | 734–753 | `setText` of `modeLabel`; `setClass` of `` `recursion-mode-pill ${modifier}` `` where `modifier` is `unwinding`, `base`, `fault`, or `''` as the ternary already computes |
| `#playPauseBtn` | 775–777 | `setText(els.playPauseBtn, state.playing ? 'Pause' : 'Play', domMemo, 'play')` |

`setText` compares against the last value passed in, not against `element.textContent`. Reading `textContent` allocates a concatenation and, on a node that still has element children, would not match the source string.

`#stageCaptionSep` is cached (`src/app.js` 71) and never written. Leave the node in the HTML. It is the static `·` between control and equation (`index.html` 128). Do not start updating it per step.

### P1.2 Cache the editor badge for the life of the source

**Where:** `src/app.js` 501–517.

`origin.line` changes with the step. The badge title does not. Every step currently does `expressions.find`, `String(state.source).match(/\bentry\s+.../)`, a whitespace collapse, and a `textContent` write of a string that is constant until the source, fuel, or stack limit changes.

Compute it once at the end of a successful `executeCurrentSource` (`src/app.js` 379, before `syncStepUI`), store `state.entryBadgeText`, and in `syncStepUI` only `setText(els.activeAstBadge, state.entryBadgeText, ...)`.

```javascript
function computeEntryBadgeText() {
  const activeMeta = PRESET_EDITORIAL_META[state.activePresetId];
  const title = activeMeta?.title || state.trace.programName || 'Sherlock Program';
  const entryExpr = state.trace.programImage?.expressions?.find((e) => e.astPath === 'entry/');
  const entryMatch = String(state.source || '').match(/\bentry\s+([\s\S]*?)\s*;/);
  const liveEntry = (entryExpr?.text || (entryMatch ? entryMatch[1] : '') || activeMeta?.sub || '')
    .replace(/\s+/g, ' ')
    .trim();
  const shortEntry = liveEntry.length > 36 ? `${liveEntry.slice(0, 35)}…` : liveEntry;
  return shortEntry ? `${title} · ${shortEntry}` : title;
}
```

A failed compile returns at `src/app.js` 373–376 without calling `syncStepUI`, so the badge stays on the previous successful program. Computing the badge only on success keeps that.

### P1.3 Line-number gutter: move one span, do not rebuild the column

**Where:** `src/app.js` 346–362. CSS: `src/styles.css` 624–646. Initial text is the single character `1` at `index.html` 90.

The early return at 348–350 is correct. The miss path builds an HTML string of every line and assigns `innerHTML`. That:

- Lays out the whole gutter on every active-line change during playback.
- Replaces every child, which resets `scrollTop` on `.line-numbers` (`overflow: hidden` at `src/styles.css` 636). The textarea scroll listener (`src/app.js` 804–808) does not re-fire, so a scrolled editor desynchronizes the gutter until the next manual scroll.

Inactive lines today are text nodes. Only the active line is `<span class="active-line">`. Keep that shape so the inline padding and left border (`src/styles.css` 639–646) stay on one line only. Giving every line the active-line padding would shift the column.

```javascript
function updateEditorLineNumbers(activeLine = 1) {
  const host = els.editorLineNumbers;
  const lines = (els.sourceEditor.value || '').split('\n').length || 1;
  const safeLine = Math.max(1, Math.min(lines, activeLine || 1));

  if (lines !== state.lastLineCount || host.childNodes.length === 0) {
    const frag = document.createDocumentFragment();
    for (let i = 1; i <= lines; i++) {
      frag.append(document.createTextNode(String(i)), document.createTextNode('\n'));
    }
    const saved = host.scrollTop;
    host.replaceChildren(frag);
    host.scrollTop = saved;
    state.lastLineCount = lines;
    state.lastActiveLine = -1;
  }

  if (safeLine === state.lastActiveLine) return;

  const lineNode = (line) => host.childNodes[(line - 1) * 2];

  if (state.lastActiveLine > 0) {
    const prev = lineNode(state.lastActiveLine);
    if (prev && prev.nodeType === Node.ELEMENT_NODE) {
      host.replaceChild(document.createTextNode(prev.textContent), prev);
    }
  }

  const next = lineNode(safeLine);
  if (next && next.nodeType === Node.TEXT_NODE) {
    const span = document.createElement('span');
    span.className = 'active-line';
    span.textContent = next.nodeValue;
    host.replaceChild(span, next);
  }
  state.lastActiveLine = safeLine;
}
```

Child layout is `(text or span, newline)` repeated, so line `i` lives at child index `(i - 1) * 2`. Two node replacements instead of `lines` parsed nodes. `scrollTop` is untouched on an active-line change.

On the textarea `scroll` listener (`804–808`), coalesce the write to one per frame. The gutter matches the textarea at paint time, which is the only moment the mismatch is visible:

```javascript
let lineScrollQueued = false;
els.sourceEditor.addEventListener('scroll', () => {
  if (lineScrollQueued) return;
  lineScrollQueued = true;
  requestAnimationFrame(() => {
    lineScrollQueued = false;
    els.editorLineNumbers.scrollTop = els.sourceEditor.scrollTop;
  });
}, { passive: true });
```

### P1.4 Cache the seven camera buttons

**Where:** `src/app.js` 335–343 and 871–877.

`updateCameraLensButtons` calls `querySelectorAll('button')` on every camera change. The seven buttons are static (`index.html` 167–175). `querySelectorAll` returns a static `NodeList`; it does not update if the DOM later changed, and nothing changes these buttons.

```javascript
const cameraLensButtons = Array.from(els.cameraLensBar.querySelectorAll('button'));

function updateCameraLensButtons(activePreset) {
  for (const btn of cameraLensButtons) {
    const isActive = btn.dataset.preset === activePreset;
    btn.classList.toggle('active', isActive);
    const pressed = isActive ? 'true' : 'false';
    if (btn.getAttribute('aria-pressed') !== pressed) btn.setAttribute('aria-pressed', pressed);
  }
  if (els.stageInspectText) {
    els.stageInspectText.textContent = getDefaultStageHint(activePreset);
  }
}

for (const btn of cameraLensButtons) {
  btn.addEventListener('click', () => {
    const preset = btn.dataset.preset;
    stage.setCameraPreset(preset);
    updateCameraLensButtons(preset);
  });
}
```

`classList.toggle(token, force)` does not mutate when the token is already in the requested state. The `aria-pressed` guard avoids a redundant attribute change on the six buttons that were already correct.

### P1.5 Do not call Quineling telemetry when its inputs did not change

**Where:** `src/app.js` 554–567. Callee: `getQuinelingTelemetry` in `src/designers/designer8_quineling.js` 275–330, which always calls `resolveCompiledQuineling` (243–253).

The compiler cache makes the repeat call cheap relative to a fresh compile, and the DOM string cache then throws the result away when the gesture matches. The gesture depends on rule line, rule id, frame count, previous frame count, active-call count, and control kind (`designer8_quineling.js` 279–314). Species text depends on the source. Cache in `app.js` with a generation bumped every successful compile, not with the whole source string:

```javascript
let sourceGeneration = 0;
let qdlCacheKey = '';
let qdlCacheValue = null;

// at the end of a successful executeCurrentSource, before syncStepUI:
sourceGeneration += 1;

// inside syncStepUI:
const qdlKey = [
  sourceGeneration,
  step.ruleLine,
  step.ruleId,
  frames.length,
  step.before?.frameCount ?? frames.length,
  (step.after?.activeCalls ?? []).length,
  rawCtrlKind,
].join('|');
if (qdlKey !== qdlCacheKey) {
  qdlCacheKey = qdlKey;
  qdlCacheValue = getQuinelingTelemetry({
    presetId: state.activePresetId,
    source: state.source,
    trace: state.trace,
    step,
  });
}
const qdlTel = qdlCacheValue;
```

Same object fields, so the P0.4 text writes produce the same `#telemetryQdlMorph` line. This does not change the compiler. Auditor #8 owns that cache.

### P1.6 Memoize `formatAlgebraicText`

**Where:** `src/app.js` 126–136. Called from control text, the equation, every slot readout, and every frame label, on every step. The function is pure. A scrub back and forth repeats the same inputs. The trace has a bounded set of distinct labels.

```javascript
const algebraicCache = new Map();
function formatAlgebraicText(str) {
  const key = String(str || '');
  const hit = algebraicCache.get(key);
  if (hit !== undefined) return hit;
  const out = key
    .replace(/\[hole\]/g, '[?]')
    .replace(/\bMul\b/g, '×')
    .replace(/\bAdd\b/g, '+')
    .replace(/\bSub\b/g, '−')
    .replace(/\bLt\b/g, '<')
    .replace(/\bEq\b/g, '==')
    .replace(/\s*\*\s*/g, ' × ')
    .replace(/\s*-\s*/g, ' − ');
  algebraicCache.set(key, out);
  return out;
}
```

Clear the map inside `executeCurrentSource` if a long editing session should not retain labels from abandoned programs. Output for a given input does not change.

### P1.7 Build the preset strip once

**Where:** `src/app.js` 300–333, called from `selectPreset` (312) and init (975).

This is not the scrub path. It still runs `innerHTML = ''`, recreates eight buttons, parses two HTML fragments per button, and binds eight new listeners on every preset click (and on keys `1`–`8`, `src/app.js` 901–906). The labels are constant (`PRESET_EDITORIAL_META`, lines 9–18).

```javascript
function mountPresetStrip() {
  els.presetStrip.replaceChildren();
  for (const p of PRESET_PROGRAMS) {
    const meta = PRESET_EDITORIAL_META[p.id] || { title: p.title, sub: p.badge };
    const btn = document.createElement('button');
    btn.type = 'button';
    btn.className = 'preset-btn';
    btn.dataset.presetId = p.id;
    const title = document.createElement('span');
    title.textContent = meta.title;
    const sub = document.createElement('small');
    sub.textContent = meta.sub;
    btn.append(title, sub);
    btn.addEventListener('click', () => selectPreset(p));
    els.presetStrip.append(btn);
  }
}

function updatePresetSelection() {
  const active = PRESET_PROGRAMS.find((p) => p.id === state.activePresetId);
  for (const btn of els.presetStrip.children) {
    const on = btn.dataset.presetId === state.activePresetId;
    const cls = on ? 'preset-btn active' : 'preset-btn';
    if (btn.className !== cls) btn.className = cls;
    const pressed = on ? 'true' : 'false';
    if (btn.getAttribute('aria-pressed') !== pressed) btn.setAttribute('aria-pressed', pressed);
  }
  if (active) els.presetDescription.textContent = active.description;
}
```

`selectPreset` then updates the editor, sliders, and fuel labels, calls `updatePresetSelection()` instead of `renderPresets()`, and compiles. `textContent` on the title and subtitle matches `escapeHtml` for these constant strings, because none of the editorial titles contain HTML.

`.preset-btn` transitions background, border, color, and shadow (`src/styles.css` 516–521). Those transitions never fire today, because the old button is discarded and the new one is born `.active`. If the same button gains `.active`, the 150 ms ease becomes visible. That is a different preset-switch picture. P2.3 removes that latent transition so the snap stays.

Hover is the `::after` opacity transition at `src/styles.css` 523–531, which is a separate property and stays.

### P1.8 Coalesce live compile to one run per frame, and drop stale results

**Where:** `src/app.js` 364–392, 800–803, 810–820.

Every `input` event on the editor calls `executeCurrentSource(false)`, and so does every fuel and stack slider tick. The function is `async` and has no generation. A slow compile of an older source can finish after a newer one and call `syncStepUI` with the old trace. The picture the user should see is the latest source, fuel, and stack limit.

Coalescing to one `requestAnimationFrame` does not change a painted frame: the browser presents one frame, and `executeCurrentSource` reads the editor and sliders at that moment. A generation counter drops a completion that is no longer current, including compiles already in flight from the previous frame.

```javascript
let compileGen = 0;
let compileQueued = false;

function scheduleCompile(resetPlayhead) {
  if (compileQueued) return;
  compileQueued = true;
  const reset = resetPlayhead;
  requestAnimationFrame(() => {
    compileQueued = false;
    executeCurrentSource(reset);
  });
}

async function executeCurrentSource(resetPlayhead = true) {
  const gen = ++compileGen;
  state.source = els.sourceEditor.value;
  const res = await compileAndRunSherlock(state.source, {
    fuel: state.fuel,
    stackLimit: state.stackLimit,
    sliceWidth: 4,
    preferRust: true,
  });
  if (gen !== compileGen) return;

  if (!res.ok) {
    els.compileErrorBanner.textContent = res.error || 'Compilation error';
    if (els.compileErrorBanner.classList.contains('hidden')) {
      els.compileErrorBanner.classList.remove('hidden');
    }
    return;
  }

  if (!els.compileErrorBanner.classList.contains('hidden')) {
    els.compileErrorBanner.classList.add('hidden');
  }
  state.trace = res;
  // ...existing playhead clamp, lastRenderedStepIdx = -1, entry badge, sourceGeneration...
  syncStepUI(Math.floor(state.playhead));
}
```

Wire it as:

- `#compileRunBtn` click and Ctrl/Cmd+Enter: `executeCurrentSource(true)` immediately. One explicit run should not wait a frame.
- `#sourceEditor` `input`: `updateEditorLineNumbers(1)` immediately (the gutter is local), then `scheduleCompile(false)`.
- Fuel and stack `input`: update `state.fuel` / `state.stackLimit` and the two labels immediately (the number under the thumb must track the drag), then `scheduleCompile(false)`.

`scheduleCompile` as written keeps the `resetPlayhead` value from the first caller in the frame. Editor and slider callers both pass `false`. Do not mix a `true` call into the same queue; the button calls `executeCurrentSource(true)` directly and bumps `compileGen`, which invalidates a queued older run when it completes.

The error banner class toggles (`374–375`, `379`) only write when the class is actually wrong.

## P2 — CSS containment and compositing

No `contain`, `content-visibility`, or `will-change` exists in `src/styles.css`. Step updates restyle text inside cards that also paint two-layer box shadows (`--md-sys-elevation-level1` at lines 40–42, used by `.panel` at 464, `.stage-caption` at 851, `.transport-bar` at 939, `.telemetry` cards, `.ring-pill.ring-active` at 1303). Containment does not change those shadows. It stops a text write in the inspector from dirtying the masthead, the primer, the canvas card, and the prose.

`contain: content` means `layout paint style`. It does not include `size`, so a percentage-sized box does not collapse. There is no `position: fixed` descendant in this CSS, so the containing-block side effect of paint containment does not move any overlay. Do not upgrade these to `contain: strict` / `contain: size`.

### P2.1 Isolate the three columns and the live widgets

Add `contain: content` to the rules that already exist. Do not change their other declarations.

| Selector | Line | Add |
| --- | --- | --- |
| `.workbench-grid` | 404 | `contain: layout;` |
| `.panel` | 461 | `contain: content;` |
| `.stage-column` | 809 | `contain: content;` |
| `.stage-viewport-card` | 815 | `contain: content;` |
| `.stage-caption` | 843 | `contain: content;` |
| `.transport-bar` | 931 | `contain: content;` |
| `.lens-bar` | 1003 | `contain: content;` |
| `.editor-container` | 573 | `contain: content;` |
| `.line-numbers` | 624 | `contain: content;` |
| `.telemetry-dl` | 1136 | `contain: content;` |
| `.six-rings-bar` | 1274 | `contain: content;` |
| `.ring-pill` | 1280 | `contain: content;` |
| `.frames-list` | 1524 | `contain: content;` |
| `.frame-item` | 1532 | `contain: content;` |
| `.recursion-calls-track` | 1633 | `contain: content;` |

`.stage-viewport-card` already has `overflow: hidden` (823) and a definite height (`min(68vh, 640px)`, line 817). `contain: content` adds layout isolation on top of that clip. The canvas stays `width/height: 100%` (`831–834`).

`.panel` padding is 16px (465) and the focus ring is 2px outside the control (`:focus-visible` at 131–134). The ring fits inside the padding, so paint containment does not clip it. `.recursion-calls-track` already clips with `overflow-y: auto` (1639).

Example, `.stage-viewport-card` (`src/styles.css` 815–824):

```css
.stage-viewport-card {
  position: relative;
  height: min(68vh, 640px);
  min-height: 464px;
  background: var(--md-sys-color-surface);
  border: 1px solid var(--md-sys-color-outline-variant);
  border-radius: var(--md-sys-shape-corner-large);
  box-shadow: var(--md-sys-elevation-level1);
  overflow: hidden;
  contain: content;
}
```

The radial gradient that is on this rule today is removed in P2.5. If that removal is deferred, keep the gradient line and only add `contain: content`.

### P2.2 Skip layout of the below-fold editorial deck

**Where:** `.prose-deck` at `src/styles.css` 1703–1709. The section starts at `index.html` 252 and runs through the 39-rule guide and both data tables (through line 561).

On a laptop-height viewport the workbench already fills the screen: the stage is at least 464px (`src/styles.css` 818), plus caption, transport, and lens bar, plus masthead and primer. The prose is the region that should not layout or paint until it is near the viewport. `content-visibility: auto` still hits find-in-page and the accessibility tree in current Chromium, and the section renders in full when scrolled into view. `contain-intrinsic-size: auto 3200px` reserves a scrollbar estimate on the first approach; after the first layout, the `auto` keyword remembers the real height so the bar does not jump again.

```css
.prose-deck {
  padding-top: 24px;
  border-top: 1px solid var(--md-sys-color-outline-variant);
  display: flex;
  flex-direction: column;
  gap: 20px;
  content-visibility: auto;
  contain-intrinsic-size: auto 3200px;
}
```

Do not put `content-visibility: auto` on `.stage-column`, `.panel-left`, or `#sourceEditor`. The stage has to paint, and the editor has to keep selection and focus. Do not put it on `table` or `tr`. It breaks row and column measurement. One rule on `.prose-deck` is the right grain.

Do not put it on `.top-primer-strip`. Those four cards are above the fold (`index.html` 34–66).

### P2.3 Delete transitions that only exist because the nodes used to be thrown away

**Where:** `src/styles.css` 1292 (`.ring-pill`), 1660 (`.rec-call-node`), and 516–521 (`.preset-btn`).

`.ring-pill` transitions `border-color`, `background-color`, and `box-shadow` over 160 ms. `.rec-call-node` transitions `border-color`, `background-color`, and `color` over 140 ms. Playback replaces the elements, so the transition never observes a class change. The on-screen step change is a snap. After P0 reuses the elements, those transitions would start running: the frontier ring would fade for 160 ms, and a call chip would fade between waiting, active, and returned. At 4× a step arrives about every 80 ms, so the fade would never settle. That is a different picture.

Hover on these controls is the `::after` opacity fade (`src/styles.css` 337–346, 523–531, 1663–1675), not the background transition. Removing the background transition leaves the hover layer.

```css
.ring-pill {
  display: grid;
  grid-template-columns: 64px minmax(0, 1fr) minmax(0, 1fr);
  align-items: stretch;
  gap: 8px;
  padding: 6px 8px;
  background: var(--md-sys-color-surface-container-low);
  border: 1px solid transparent;
  border-radius: var(--md-sys-shape-corner-small);
  font-family: var(--font-mono);
  font-size: 11.5px;
  min-width: 0;
  contain: content;
}

.rec-call-node {
  position: relative;
  isolation: isolate;
  overflow: hidden;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 4px;
  font-family: var(--font-mono);
  font-size: 12px;
  font-variant-numeric: tabular-nums;
  min-height: 36px;
  padding: 6px 12px;
  border-radius: var(--md-sys-shape-corner-small);
  background: var(--md-sys-color-surface-container-low);
  border: 1px solid var(--md-sys-color-outline);
  color: var(--ink-soft);
  cursor: pointer;
}
```

On `.preset-btn` (516–521), delete the `transition` of `background-color`, `border-color`, `color`, and `box-shadow`. Leave the `::after` opacity transition (530). Preset activation stays a snap, hover stays a fade.

The scrubber thumb shadow transition (`src/styles.css` 770 and 781) is a real hover/active animation on an element that is not rebuilt. Leave it. `contain: content` on `.transport-bar` limits that 10px ring shadow (`786–805`) to the transport bar while the user drags.

### P2.4 Author rule for pooled `[hidden]`

**Where:** add next to `.overflow-badge` (`src/styles.css` 1451) and the frame / call rules.

`.frame-item` is `display: flex` (1533). `.rec-call-node` is `display: inline-flex` (1646). `.overflow-badge` is `display: inline-flex` (1452). An author `display` can beat a non-`!important` UA `[hidden]` rule. The pool uses the `hidden` attribute to mean "not in the layout", which is what omitting the node from `innerHTML` means today.

```css
.frame-item[hidden],
.rec-call-node[hidden],
.rec-call-arrow[hidden],
.overflow-badge[hidden],
.frames-list > .empty-note[hidden],
.recursion-calls-track > .empty-note[hidden] {
  display: none !important;
}
```

### P2.5 Do not promote the canvas with `will-change` or `translateZ(0)`

**Where:** `#sherlockCanvas` at `src/styles.css` 831–836. The renderer is a `WebGPURenderer` on that canvas (`src/stage/SherlockStage.js` 97–101) with an opaque scene clear, `scene.background = 0xf4f0e8` (`SherlockStage.js` 90). Device pixel ratio is capped at 2 (`SherlockStage.js` 102–103).

A canvas that the GPU presents is already its own compositor surface. `will-change: transform` or `transform: translateZ(0)` adds another layer and another scale. When the CSS size and the drawing-buffer size disagree by a fraction of a pixel, that extra layer softens the image. That fails the pixel constraint. Do not add it.

`contain: content` on `.stage-viewport-card` (P2.1) is the isolation that does not resample the buffer.

The card background is a radial vignette over `--md-sys-color-surface` (`src/styles.css` 819):

```css
background: radial-gradient(ellipse at center, transparent 0 42%, rgba(24, 24, 22, 0.04) 78%, transparent 100%), var(--md-sys-color-surface);
```

The canvas covers `.canvas-holder`, which covers the card's content box (`826–834`). The scene clear is opaque and the same paper hue. The vignette does not show through an opaque canvas, and the 1px border covers the border-box edge. Replacing that `background` with `background: var(--md-sys-color-surface);` removes a full-card gradient paint on every card invalidation and does not change the presented stage.

Keep the gradient if a later change makes the canvas clear transparent. Under the current renderer it is covered.

### P2.6 What not to change in CSS

| Idea | Why it does not qualify |
| --- | --- |
| Remove elevation shadows | Changes the M3 cards. Containment stops them repainting; it does not require deleting them. |
| Animate ring background with opacity crossfades | Different motion than today's snap, and more layers. |
| `overflow-wrap: anywhere` on `.stage-equation` (909) and `.telemetry-dl dd` (1164) | Changing wrap changes line breaks of long control strings. |
| `content-visibility` on the stage, editor, or table rows | Skips the live picture, or breaks table layout. |
| `touch-action: none` on the canvas | Stops a page scroll that starts on the canvas. That is a gesture change. |
| Defer the Google font stylesheet | `display=swap` is already set (`index.html` 10). An async font CSS load adds a second fallback flash. `preconnect` to `fonts.googleapis.com` and `fonts.gstatic.com` is already present (8–9). |
| `font-display` metric overrides / `size-adjust` | Changes the fallback paint. The settled webfont paint is already the target. Leave it. |

## P3 — `modulepreload` the real graph

**Where:** `index.html` 3–21 and 564.

The module script is the last line of the body. A module is deferred either way, but the fetch starts when the parser reaches the tag. The document above it is the whole editorial page, so `app.js` does not start until that HTML is tokenized. Its imports are then discovered one file at a time.

Static graph from `src/app.js` 1–7 and `src/stage/SherlockStage.js` 1–10, with sizes measured in this tree:

| URL | Bytes | Hop |
| --- | --- | --- |
| `/src/app.js` | 35 KB | 0 |
| `/src/engine/sherlockEngine.js` | 71 KB | 1 |
| `/src/stage/SherlockStage.js` | 13 KB | 1 |
| `/src/designers/designer8_quineling.js` | 70 KB | 1, also via the stage |
| `/vendor/three.webgpu.js` | 2.2 MB | 2 |
| `/vendor/three.core.js` | 1.4 MB | 3, imported by `three.webgpu.js` line 6 |
| `/src/designers/theme.js` | 19 KB | 2 |
| `/src/designers/designer1_package.js` | 30 KB | 2 |
| `/src/designers/designer2_well.js` | 108 KB | 2 |
| `/src/designers/designer3_frames.js` | 26 KB | 2 |
| `/src/designers/designer4_arena.js` | 16 KB | 2 |
| `/src/designers/designer5_circuit.js` | 9.8 KB | 2 |
| `/src/designers/designer6_loom.js` | 6.7 KB | 2 |
| `/src/designers/designer7_cinema.js` | 31 KB | 2 |
| `/src/engine/quinelingsSdk.js` | 150 KB | 2, via designer 8 |
| `/src/engine/sherlockQuinelingCompiler.js` | 22 KB | 2, via designer 8 |

`three.webgpu.js` does not import `three.tsl.js`. Nothing under `src/` imports `three/tsl` or `three.webgpu.nodes.js`. The import map entry `"three/tsl"` (`index.html` 17) is unused. Preloading `/vendor/three.tsl.js` (36 KB) or `/vendor/three.webgpu.nodes.js` (2.2 MB) would compete with the files that actually run. Do not add those hints.

`modulepreload` of a parent does not start a child download until the parent has arrived and been parsed. For a 2.2 MB parent, that hides the 1.4 MB core file for the whole transfer. List `three.core.js` and `three.webgpu.js` explicitly. Same-origin module scripts are already CORS mode; do not add `crossorigin="anonymous"` on these links. A credentials-mode mismatch refetches the module.

Put this in `<head>` after the import map (`index.html` 12–20) and move the module script up with it. A `type="module"` script is deferred, so top-level `getElementById` in `src/app.js` 56–97 still runs after the document is parsed. Behavior of `els` does not change.

```html
<link rel="modulepreload" href="/vendor/three.core.js" fetchpriority="high" />
<link rel="modulepreload" href="/vendor/three.webgpu.js" fetchpriority="high" />
<link rel="modulepreload" href="/src/engine/sherlockEngine.js" />
<link rel="modulepreload" href="/src/engine/quinelingsSdk.js" />
<link rel="modulepreload" href="/src/engine/sherlockQuinelingCompiler.js" />
<link rel="modulepreload" href="/src/stage/SherlockStage.js" />
<link rel="modulepreload" href="/src/designers/theme.js" />
<link rel="modulepreload" href="/src/designers/designer1_package.js" />
<link rel="modulepreload" href="/src/designers/designer2_well.js" />
<link rel="modulepreload" href="/src/designers/designer3_frames.js" />
<link rel="modulepreload" href="/src/designers/designer4_arena.js" />
<link rel="modulepreload" href="/src/designers/designer5_circuit.js" />
<link rel="modulepreload" href="/src/designers/designer6_loom.js" />
<link rel="modulepreload" href="/src/designers/designer7_cinema.js" />
<link rel="modulepreload" href="/src/designers/designer8_quineling.js" />
<link rel="modulepreload" href="/src/app.js" />
<script type="module" src="/src/app.js"></script>
```

Delete the copy at `index.html` 564. Keep one module script. `fetchpriority="high"` belongs on the two multi-megabyte files only, so the designer modules do not outrank them.

`/src/styles.css` is already a blocking stylesheet in the head (`index.html` 11). A preload of that same URL does not move the first paint. Leave it.

This does not change the evaluated module graph, the import map, or any rendered pixel. It starts the same fetches during HTML parse instead of after each parent is parsed.

## Application order

1. P3 `modulepreload` and the deferred script move. Independent of the DOM work.
2. P2.1, P2.2, P2.4, P2.5. Containment and the hidden-attribute rule. No DOM contract change.
3. P1.1 helpers, then P0.4 telemetry split. Smallest DOM change, easiest to diff visually.
4. P0.1, P0.2, P0.3 together with P2.3. Pools and transition removal in one patch, or the rings and call chips will fade.
5. P0.5 scrub coalesce, P1.3 gutter, P1.8 compile coalesce.
6. P1.2, P1.4, P1.5, P1.6, P1.7. Pure caches.

After P0, `state.lastRingsHtml`, `lastFramesHtml`, `lastCallsHtml`, and the five `last*TelemetryHtml` fields (`src/app.js` 31–40) are unused. Remove them so the previous multi-kilobyte HTML strings are not retained.

## Checks that confirm the picture did not move

These are the comparisons that matter. This audit did not re-run them in a browser.

- Factorial, step 1 and the `Done` step: caption badge, control text, equation, scrubber label, rule pill, and the seven telemetry lines match the current strings, including the middle dot and the `<small>` color.
- A mid-trace step with both slots filled: West A / East B class colors (teal, cyan, amber, gold, violet), frontier outline, and `R1 (02)` … `R6 (12)` labels match.
- Stack-overflow preset: `R6.B` shows the red overflow row and the `+N overflow` badge; other rings do not.
- Done / Rejected / Exhausted: the stator chip uses `is-done` or `is-fault`, and the status badge uses the same four classes as `getStepStatusInfo` (`src/app.js` 394–408).
- Call chips still jump via the existing delegated listener (`780–790`). Keyboard focus on a chip survives the next step.
- Editor scrolled to the bottom, then playback moves the active line: gutter does not jump back to line 1, and the active line still has the amber left border on that line only.
- Preset click and keys `1`–`8`: the active card snaps, it does not fade, and hover still fades the state layer.
- Dragging the scrubber: the thumb tracks the pointer, the inspector updates once per frame, and the final step matches the released value.
- Camera segmented control: `aria-pressed` and `.active` match `data-preset`. The canvas image is as sharp as before (no new transform layer).
- Scrolling to the architecture guide: the 39-rule cards and both tables render in full. The scrollbar does not jump after the first reveal.
- Network panel, cold load: `three.core.js` and `three.webgpu.js` start during HTML parse, in parallel. `three.tsl.js` and `three.webgpu.nodes.js` are not requested.

## Not in this pass

- `stage.render` running while paused, shadow maps, and raycasting. Those live in `src/stage/SherlockStage.js` (auditor #1).
- `buildStepState` / `buildRecursionTelemetry` allocation inside `src/engine/sherlockEngine.js` (auditor #9), except the app-level cache at `src/app.js` 910–942 which should stay.
- Quineling compiler memoization itself (auditor #8). P1.5 only stops the DOM path from calling it.
- Rewriting inspector copy, type size, or color. Those change the picture this pass is required to keep.
