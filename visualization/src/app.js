import {
  PRESET_PROGRAMS,
  compileAndRunSherlock,
  buildStepState,
} from './engine/sherlockEngine.js';
import { SherlockStage } from './stage/SherlockStage.js';
import { getQuinelingTelemetry } from './designers/designer8_quineling.js';

const PRESET_EDITORIAL_META = {
  factorial: { title: 'Factorial', sub: 'fact(4)' },
  list_fold: { title: 'List sum', sub: 'sum_list([5, 8, 3])' },
  tree_fold: { title: 'Tree sum', sub: 'sum_tree(branch(…))' },
  mutual_even_odd: { title: 'Mutual recursion', sub: 'is_even(4)' },
  lexical_let: { title: 'Lexical let', sub: 'cube_plus(12, 272)' },
  checked_underflow: { title: 'Checked arithmetic', sub: 'clamp_sub(3, 9)' },
  stack_overflow: { title: 'Frame limit', sub: 'dive(6) · limit 5' },
  fuel_exhaustion: { title: 'Fuel limit', sub: 'count_down(5) · fuel 14' },
};

const state = {
  activePresetId: 'factorial',
  source: PRESET_PROGRAMS[0].source,
  fuel: PRESET_PROGRAMS[0].defaultFuel,
  stackLimit: PRESET_PROGRAMS[0].defaultStackLimit,
  trace: null,
  playhead: 0,
  playing: true,
  speed: 1.0,
  lastTick: performance.now(),
  lastRenderedStepIdx: -1,
  lastRingsHtml: '',
  lastFramesHtml: '',
  lastCallsHtml: '',
  lastActiveLine: -1,
  lastLineCount: -1,
  lastTransitionHtml: '',
  lastQdlMorphHtml: '',
  lastFramesTelemetryHtml: '',
  lastCallsTelemetryHtml: '',
  lastFuelTelemetryHtml: '',
};

const CONTROL_STRIP_SECTORS = [
  { key: 'Eval', badge: 'EVAL', title: 'Control::Eval(expr)' },
  { key: 'Push', badge: 'PUSH', title: 'Control::Push(frame, next)' },
  { key: 'Return', badge: 'RET', title: 'Control::Return(val)' },
  { key: 'Invoke', badge: 'INVOKE', title: 'Control::Invoke(name, args)' },
  { key: 'Enter', badge: 'ENTER', title: 'Control::Enter(func, args)' },
  { key: 'Bound', badge: 'BOUND', title: 'Control::Bound(func, env)' },
  { key: 'Cases', badge: 'CASES', title: 'Control::Cases(scrutinee, arms)' },
  { key: 'CaseResult', badge: 'MATCH', title: 'Control::CaseResult(matched)' },
  { key: 'PrimitiveResult', badge: 'PRIM', title: 'Control::PrimitiveResult(val)' },
  { key: 'Error', badge: 'ERR', title: 'Control::Error(code)' },
];

const els = {
  presetStrip: document.getElementById('presetStrip'),
  presetDescription: document.getElementById('presetDescription'),
  sourceEditor: document.getElementById('sourceEditor'),
  editorLineNumbers: document.getElementById('editorLineNumbers'),
  compileRunBtn: document.getElementById('compileRunBtn'),
  compileErrorBanner: document.getElementById('compileErrorBanner'),
  fuelSlider: document.getElementById('fuelSlider'),
  fuelValLabel: document.getElementById('fuelValLabel'),
  stackSlider: document.getElementById('stackSlider'),
  stackValLabel: document.getElementById('stackValLabel'),
  activeAstBadge: document.getElementById('activeAstBadge'),
  activeSpanBadge: document.getElementById('activeSpanBadge'),
  stageStatusBadge: document.getElementById('stageStatusBadge'),
  hudControlSummary: document.getElementById('hudControlSummary'),
  stageCaptionSep: document.getElementById('stageCaptionSep'),
  recursionEquationText: document.getElementById('recursionEquationText'),
  stageInspectText: document.getElementById('stageInspectText'),
  playPauseBtn: document.getElementById('playPauseBtn'),
  stepPrevBtn: document.getElementById('stepPrevBtn'),
  stepNextBtn: document.getElementById('stepNextBtn'),
  resetStepBtn: document.getElementById('resetStepBtn'),
  scrubberStepLabel: document.getElementById('scrubberStepLabel'),
  stepScrubber: document.getElementById('stepScrubber'),
  speedSelect: document.getElementById('speedSelect'),
  cameraLensBar: document.getElementById('cameraLensBar'),
  telemetryRuleBadge: document.getElementById('telemetryRuleBadge'),
  telemetryStatus: document.getElementById('telemetryStatus'),
  telemetryTransition: document.getElementById('telemetryTransition'),
  telemetryControl: document.getElementById('telemetryControl'),
  telemetryQdlMorph: document.getElementById('telemetryQdlMorph'),
  telemetryFrames: document.getElementById('telemetryFrames'),
  telemetryCalls: document.getElementById('telemetryCalls'),
  telemetryFuel: document.getElementById('telemetryFuel'),
  ringsSummaryBadge: document.getElementById('ringsSummaryBadge'),
  sixRingsGauge: document.getElementById('sixRingsGauge'),
  frameStackCount: document.getElementById('frameStackCount'),
  frameStackList: document.getElementById('frameStackList'),
  recursionModePill: document.getElementById('recursionModePill'),
  recursionCallsTrack: document.getElementById('recursionCallsTrack'),
  canvas: document.getElementById('sherlockCanvas'),
};

function getDefaultStageHint(preset = stage?.activePreset) {
  if (typeof preset === 'string' && preset.startsWith('interior')) {
    return 'Drag up/down to see Rings & 3D Floor · Scroll to rise/descend · Shift-drag to move';
  }
  return 'Drag to orbit · Scroll or pinch to zoom';
}

const stage = new SherlockStage(els.canvas, {
  onInspect: (meta) => {
    if (!els.stageInspectText) return;
    if (!meta) {
      els.stageInspectText.textContent = getDefaultStageHint(stage?.activePreset);
      return;
    }
    const title = meta.title || meta.id || 'Die component';
    const sub = meta.subtitle || (typeof meta.details === 'string' ? meta.details : '');
    els.stageInspectText.textContent = sub ? `${title} — ${sub}` : title;
  },
});

function escapeHtml(str) {
  return String(str)
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;');
}

function formatAlgebraicText(str) {
  return String(str || '')
    .replace(/\[hole\]/g, '[?]')
    .replace(/\bMul\b/g, '×')
    .replace(/\bAdd\b/g, '+')
    .replace(/\bSub\b/g, '−')
    .replace(/\bLt\b/g, '<')
    .replace(/\bEq\b/g, '==')
    .replace(/\s*\*\s*/g, ' × ')
    .replace(/\s*-\s*/g, ' − ');
}

function formatShortEnvHint(fr, fallbackEnvHint = '') {
  if (fr && Array.isArray(fr.bindings)) {
    if (fr.bindings.length === 0) return 'root';
    return fr.bindings
      .slice(0, 2)
      .map((b) => `${b.name}=${b.valueSummary}`)
      .join(', ');
  }
  return fallbackEnvHint;
}

function getFrameSlotMeta(fr, fallbackEnvHint = '') {
  if (!fr) {
    return {
      cls: 'slot-idle',
      itemCls: '',
      family: 'IDLE',
      readout: '—',
      tooltip: 'Idle half-ring arc',
    };
  }

  const rawLabel = String(fr.label || '');
  const normalizedOp = formatAlgebraicText(rawLabel);
  const envHint = formatShortEnvHint(fr, fallbackEnvHint);
  const envSuffix = envHint ? ` (${envHint})` : '';

  if (fr.kind === 'Return') {
    const retType = rawLabel.replace(/^return\s*:\s*/i, '').trim() || 'Val';
    return {
      cls: 'slot-env slot-env-return',
      itemCls: 'is-env',
      family: 'ENV',
      readout: `RET:${retType}${envSuffix}`,
      tooltip: `ENV · Return(${retType})${envSuffix} — Teal #0d9488`,
    };
  }

  if (fr.kind === 'Scope') {
    const bCount = fr.bindings ? fr.bindings.length : 0;
    const scopeBase = bCount > 0 ? `SCOPE(${bCount})` : 'SCOPE';
    return {
      cls: 'slot-env slot-env-scope',
      itemCls: 'is-env is-env-scope',
      family: 'ENV',
      readout: `${scopeBase}${envSuffix}`,
      tooltip: `ENV · Scope (${bCount} bindings)${envSuffix} — Sky-Cyan #0284c7`,
    };
  }

  if (fr.kind === 'Right') {
    return {
      cls: 'slot-op slot-op-right',
      itemCls: 'is-operand is-op is-op-right',
      family: 'OP·R',
      readout: `${normalizedOp}${envSuffix}`,
      tooltip: `OP · Right(${normalizedOp})${envSuffix} — Gold #b45309 (Rule 149 in-place rewrite)`,
    };
  }

  if (fr.kind === 'Left') {
    return {
      cls: 'slot-op slot-op-left',
      itemCls: 'is-operand is-op',
      family: 'OP·L',
      readout: `${normalizedOp}${envSuffix}`,
      tooltip: `OP · Left(${normalizedOp})${envSuffix} — Amber #d97706 (Rule 117 push)`,
    };
  }

  if (fr.kind === 'Args') {
    const shortArgs = formatAlgebraicText(rawLabel.replace(/\s+args\)/i, ')'));
    return {
      cls: 'slot-op slot-op-left',
      itemCls: 'is-operand is-op',
      family: 'OP',
      readout: `${shortArgs}${envSuffix}`,
      tooltip: `OP · Args(${rawLabel})${envSuffix} — Amber #d97706`,
    };
  }

  if (fr.kind === 'Unary') {
    return {
      cls: 'slot-op slot-op-left',
      itemCls: 'is-operand is-op',
      family: 'OP',
      readout: `${normalizedOp}${envSuffix}`,
      tooltip: `OP · Unary(${normalizedOp})${envSuffix} — Amber #d97706`,
    };
  }

  if (fr.kind === 'If') {
    return {
      cls: 'slot-gate',
      itemCls: 'is-gate',
      family: 'GATE',
      readout: `IF [?]${envSuffix}`,
      tooltip: `GATE · If (${fr.detail || 'branch'})${envSuffix}`,
    };
  }

  if (fr.kind === 'Match') {
    return {
      cls: 'slot-gate',
      itemCls: 'is-gate',
      family: 'GATE',
      readout: `MATCH [?]${envSuffix}`,
      tooltip: `GATE · ${rawLabel}${envSuffix}`,
    };
  }

  if (fr.kind === 'Let') {
    const varName = rawLabel.replace(/^let\s+/i, '').replace(/\s*=.*$/, '').trim() || 'x';
    return {
      cls: 'slot-gate',
      itemCls: 'is-gate',
      family: 'GATE',
      readout: `LET ${varName}=[?]${envSuffix}`,
      tooltip: `GATE · Let ${varName} = [?]${envSuffix}`,
    };
  }

  return {
    cls: 'slot-op slot-op-left',
    itemCls: 'is-operand is-op',
    family: 'OP',
    readout: normalizedOp || fr.kind,
    tooltip: `${fr.kind}: ${normalizedOp}`,
  };
}

function formatControlDisplay(step) {
  if (!step) return 'Ready';
  const raw = step.controlSummary || '';
  const kind = step.after?.control?.kind || step.controlKind || 'Eval';
  if (
    kind === 'Eval' &&
    !raw.startsWith('Eval(') &&
    !raw.startsWith('Done(') &&
    !raw.startsWith('Rejected') &&
    !raw.startsWith('Exhausted')
  ) {
    return `Eval(${formatAlgebraicText(raw)})`;
  }
  return formatAlgebraicText(raw);
}

function formatOutcomePill(outcome) {
  if (!outcome) return 'Ready';
  if (outcome.kind === 'Done') {
    const val = outcome.summary || '';
    return val.startsWith('Done') ? val : `Done (${val})`;
  }
  if (outcome.kind === 'Rejected') {
    return outcome.code !== undefined ? `Rejected (error ${outcome.code})` : outcome.summary || 'Rejected';
  }
  if (outcome.kind === 'Exhausted') {
    return 'Exhausted';
  }
  return outcome.summary || outcome.kind || 'Ready';
}

function selectPreset(preset) {
  if (!preset) return;
  state.activePresetId = preset.id;
  state.source = preset.source;
  state.fuel = preset.defaultFuel;
  state.stackLimit = preset.defaultStackLimit;
  els.sourceEditor.value = state.source;
  els.fuelSlider.value = String(state.fuel);
  els.fuelValLabel.textContent = String(state.fuel);
  els.stackSlider.value = String(state.stackLimit);
  els.stackValLabel.textContent = String(state.stackLimit);
  els.presetDescription.textContent = preset.description;
  renderPresets();
  executeCurrentSource(true);
}

function renderPresets() {
  els.presetStrip.innerHTML = '';
  for (const p of PRESET_PROGRAMS) {
    const meta = PRESET_EDITORIAL_META[p.id] || { title: p.title, sub: p.badge };
    const isActive = p.id === state.activePresetId;
    const btn = document.createElement('button');
    btn.type = 'button';
    btn.className = `preset-btn ${isActive ? 'active' : ''}`;
    btn.setAttribute('aria-pressed', String(isActive));
    btn.innerHTML = `<span>${escapeHtml(meta.title)}</span><small>${escapeHtml(meta.sub)}</small>`;
    btn.addEventListener('click', () => selectPreset(p));
    els.presetStrip.appendChild(btn);
  }
  const activePreset = PRESET_PROGRAMS.find((p) => p.id === state.activePresetId);
  if (activePreset) {
    els.presetDescription.textContent = activePreset.description;
  }
}

function updateCameraLensButtons(activePreset) {
  for (const btn of els.cameraLensBar.querySelectorAll('button')) {
    const isActive = btn.dataset.preset === activePreset;
    btn.classList.toggle('active', isActive);
    btn.setAttribute('aria-pressed', String(isActive));
  }
  if (els.stageInspectText) {
    els.stageInspectText.textContent = getDefaultStageHint(activePreset);
  }
}

function updateEditorLineNumbers(activeLine = 1) {
  const lines = (els.sourceEditor.value || '').split('\n').length;
  if (lines === state.lastLineCount && activeLine === state.lastActiveLine) {
    return;
  }
  state.lastLineCount = lines;
  state.lastActiveLine = activeLine;
  let html = '';
  for (let i = 1; i <= lines; i++) {
    if (i === activeLine) {
      html += `<span class="active-line">${i}</span>\n`;
    } else {
      html += `${i}\n`;
    }
  }
  els.editorLineNumbers.innerHTML = html;
}

async function executeCurrentSource(resetPlayhead = true) {
  state.source = els.sourceEditor.value;
  const res = await compileAndRunSherlock(state.source, {
    fuel: state.fuel,
    stackLimit: state.stackLimit,
    sliceWidth: 4,
    preferRust: true,
  });

  if (!res.ok) {
    els.compileErrorBanner.textContent = res.error || 'Compilation error';
    els.compileErrorBanner.classList.remove('hidden');
    return;
  }

  els.compileErrorBanner.classList.add('hidden');
  state.trace = res;

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

function getStepStatusInfo(step) {
  if (!step) {
    return { label: 'Evaluating', badgeClass: 'status-evaluating', ddClass: 'is-evaluating' };
  }
  if (step.ruleLine === 80) {
    return { label: 'Done', badgeClass: 'status-done', ddClass: 'is-done' };
  }
  if (step.ruleLine === 82 || step.controlKind === 'Error') {
    return { label: 'Rejected', badgeClass: 'status-rejected', ddClass: 'is-stopped' };
  }
  if (step.ruleLine === 84) {
    return { label: 'Exhausted', badgeClass: 'status-exhausted', ddClass: 'is-stopped' };
  }
  return { label: 'Evaluating', badgeClass: 'status-evaluating', ddClass: 'is-evaluating' };
}

function resolveFrameEnvText(frames, idx, activeBindings) {
  const fr = frames[idx];
  if (fr.bindings && fr.bindings.length > 0) {
    return fr.bindings.map((b) => `${b.name} = ${b.valueSummary}`).join(', ');
  }
  // If a pending operand frame (e.g. Right or Unary) doesn't store its own env map, borrow enclosing bindings
  for (let j = idx - 1; j >= 0; j--) {
    if (frames[j].bindings && frames[j].bindings.length > 0) {
      return frames[j].bindings.map((b) => `${b.name} = ${b.valueSummary}`).join(', ');
    }
  }
  if (activeBindings && activeBindings.length > 0) {
    return activeBindings.map((b) => `${b.name} = ${b.valueSummary}`).join(', ');
  }
  return fr.detail || 'root scope';
}

function resolveChronoEnvHint(chronoFrames, chronoIdx, activeBindings) {
  const fr = chronoFrames[chronoIdx];
  if (!fr) return '';
  if (Array.isArray(fr.bindings)) {
    if (fr.bindings.length === 0) return 'root';
    return fr.bindings
      .slice(0, 2)
      .map((b) => `${b.name}=${b.valueSummary}`)
      .join(', ');
  }
  if (activeBindings && activeBindings.length > 0) {
    return activeBindings
      .slice(0, 2)
      .map((b) => `${b.name}=${b.valueSummary}`)
      .join(', ');
  }
  for (let j = chronoIdx - 1; j >= 0; j--) {
    const prev = chronoFrames[j];
    if (prev && Array.isArray(prev.bindings) && prev.bindings.length > 0) {
      return prev.bindings
        .slice(0, 2)
        .map((b) => `${b.name}=${b.valueSummary}`)
        .join(', ');
    }
  }
  return '';
}

function syncStepUI(stepIdx, prebuiltStepState = null) {
  if (!state.trace || !state.trace.steps.length) return;
  const totalSteps = state.trace.steps.length;
  const clampedIdx = Math.max(0, Math.min(totalSteps - 1, stepIdx));
  state.lastRenderedStepIdx = clampedIdx;

  const step = state.trace.steps[clampedIdx];
  const stepState = prebuiltStepState || getOrBuildStepState(clampedIdx, 0, 0.016);

  const statusInfo = getStepStatusInfo(step);
  const controlText = formatControlDisplay(step);
  const rec = stepState.recursion;
  const equationText = formatAlgebraicText(rec?.equation || step.controlSummary || '');

  // 1. Stage Caption Bar (below canvas)
  if (els.stageStatusBadge) {
    if (els.stageStatusBadge.textContent !== statusInfo.label) {
      els.stageStatusBadge.textContent = statusInfo.label;
    }
    els.stageStatusBadge.className = `status-badge ${statusInfo.badgeClass}`;
  }
  if (els.hudControlSummary) {
    els.hudControlSummary.textContent = controlText;
  }
  if (els.recursionEquationText) {
    els.recursionEquationText.textContent = equationText;
  }

  // 2. Scrubber Step Label & Transport Bound States
  if (els.scrubberStepLabel) {
    els.scrubberStepLabel.textContent = `Step ${clampedIdx + 1} / ${totalSteps}`;
  }
  if (els.stepScrubber) {
    els.stepScrubber.value = String(clampedIdx);
    els.stepScrubber.setAttribute(
      'aria-valuetext',
      `Step ${clampedIdx + 1} of ${totalSteps}, ${statusInfo.label}, Rule ${step.ruleId}`
    );
  }
  if (els.stepPrevBtn) {
    els.stepPrevBtn.disabled = clampedIdx <= 0;
  }
  if (els.stepNextBtn) {
    els.stepNextBtn.disabled = clampedIdx >= totalSteps - 1;
  }

  // 3. Active Source Line & Provenance
  const origin = step.after.controlOrigin;
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
  if (els.activeSpanBadge) {
    els.activeSpanBadge.textContent = `Line ${origin.line}`;
  }
  updateEditorLineNumbers(origin.line);

  // 4. Right Rail: Inspector Telemetry DL
  const frames = step.after.frames || [];
  const chronoFrames = [...frames].reverse();
  const activeBindings = step.after.envRoots?.[0]?.bindings || [];
  const litRings = Math.min(6, Math.ceil(chronoFrames.length / 2));
  const overflowFrames = Math.max(0, chronoFrames.length - 12);
  const initialFuel = state.trace.steps[0]?.before?.fuel ?? state.fuel;
  const fuelPct =
    initialFuel > 0 ? Math.max(0, Math.min(100, Math.round((step.after.fuel / initialFuel) * 100))) : 0;
  const rawCtrlKind = String(step.after?.control?.kind || step.controlKind || 'Eval');
  const ctrlKindUpper = rawCtrlKind.toUpperCase();
  const activeCallCount = rec
    ? rec.calls.filter((c) => c.status === 'active' || c.status === 'waiting').length
    : 0;

  els.telemetryRuleBadge.textContent = `Rule ${step.ruleId}`;

  if (els.telemetryStatus) {
    els.telemetryStatus.textContent =
      statusInfo.label === 'Done' && state.trace.outcome?.summary
        ? `Done (${state.trace.outcome.summary})`
        : statusInfo.label;
    els.telemetryStatus.className = statusInfo.ddClass;
  }
  if (els.telemetryTransition) {
    const transitionHtml = `${clampedIdx + 1} / ${totalSteps} <small>Rule ${escapeHtml(step.ruleId)} &middot; ${escapeHtml(step.ruleName)}</small>`;
    if (transitionHtml !== state.lastTransitionHtml) {
      state.lastTransitionHtml = transitionHtml;
      els.telemetryTransition.innerHTML = transitionHtml;
    }
  }
  if (els.telemetryControl) {
    els.telemetryControl.textContent = controlText;
  }

  // Live QDL Quineling Core Morphogenesis Telemetry
  const qdlTel = getQuinelingTelemetry({
    presetId: state.activePresetId,
    source: state.source,
    trace: state.trace,
    step,
  });
  if (els.telemetryQdlMorph) {
    const qdlHtml = `<strong>${escapeHtml(qdlTel.speciesName)}</strong> <small>&middot; ${escapeHtml(qdlTel.family)} (${escapeHtml(qdlTel.mechanism)}) &middot; ${escapeHtml(qdlTel.morphGesture)}</small>`;
    if (qdlHtml !== state.lastQdlMorphHtml) {
      state.lastQdlMorphHtml = qdlHtml;
      els.telemetryQdlMorph.innerHTML = qdlHtml;
    }
  }

  if (els.telemetryFrames) {
    const ringNote =
      overflowFrames > 0
        ? `${Math.min(12, chronoFrames.length)}/12 slots (${litRings}/6 rings, +${overflowFrames} overflow)`
        : `${chronoFrames.length}/12 slots (${litRings}/6 rings)`;
    const framesTelemetryHtml = `${frames.length} <small>&middot; ${ringNote}</small>`;
    if (framesTelemetryHtml !== state.lastFramesTelemetryHtml) {
      state.lastFramesTelemetryHtml = framesTelemetryHtml;
      els.telemetryFrames.innerHTML = framesTelemetryHtml;
    }
  }
  if (els.telemetryCalls) {
    const callsTelemetryHtml = `${activeCallCount} <small>&middot; ${rec ? rec.calls.length : 0} total</small>`;
    if (callsTelemetryHtml !== state.lastCallsTelemetryHtml) {
      state.lastCallsTelemetryHtml = callsTelemetryHtml;
      els.telemetryCalls.innerHTML = callsTelemetryHtml;
    }
  }
  if (els.telemetryFuel) {
    const fuelTelemetryHtml = `${step.after.fuel} / ${initialFuel} <small>&middot; charge ${step.charge}</small>`;
    if (fuelTelemetryHtml !== state.lastFuelTelemetryHtml) {
      state.lastFuelTelemetryHtml = fuelTelemetryHtml;
      els.telemetryFuel.innerHTML = fuelTelemetryHtml;
    }
  }

  // 5. 6 Stacked Horizontal Split-Arc Ring Rows (West Slot A | East Slot B) + 10-Sector Control Strip
  if (els.sixRingsGauge && els.ringsSummaryBadge) {
    const visibleSlots = Math.min(12, chronoFrames.length);
    els.ringsSummaryBadge.textContent =
      overflowFrames > 0
        ? `${visibleSlots}/12 (+${overflowFrames}) · FUEL ${fuelPct}% · CTRL: ${ctrlKindUpper}`
        : `${visibleSlots}/12 SLOTS · FUEL ${fuelPct}% · CTRL: ${ctrlKindUpper}`;

    const frontierChronoIdx = chronoFrames.length > 0 ? Math.min(11, chronoFrames.length - 1) : -1;
    const activeRingIdx = frontierChronoIdx >= 0 ? Math.floor(frontierChronoIdx / 2) : -1;

    let ringsHtml = '';
    for (let r = 0; r < 6; r++) {
      const idxA = r * 2;
      const idxB = r * 2 + 1;
      const slotLo = String(idxA + 1).padStart(2, '0');
      const slotHi = String(idxB + 1).padStart(2, '0');
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
      const ringStateCls = fill === 0 ? 'ring-idle' : isFocusRing ? 'ring-active' : 'ring-occupied';
      const titleText = `Ring R${r + 1} (Chrono Slots ${slotLo}–${slotHi}) · West A: ${metaA.readout} | East B: ${metaB.readout}`;

      ringsHtml += `
        <div class="ring-pill ${ringStateCls}" title="${escapeHtml(titleText)}">
          <div class="ring-pill-top">
            <span class="ring-id">R${r + 1} (${slotHi})</span>
            <span class="ring-slots-idx">${slotLo}–${slotHi}</span>
          </div>
          <div class="ring-slot-row ${metaA.cls} ${isFrontierA ? 'slot-frontier' : ''}" ${isFrontierA ? 'aria-current="true"' : ''} title="West Slot A (#${slotLo}): ${escapeHtml(metaA.tooltip)}">
            <span class="slot-main">
              <span class="slot-key">West A:</span>
              <span class="slot-val">${escapeHtml(metaA.readout)}</span>
            </span>
          </div>
          <div class="ring-slot-row ${metaB.cls} ${isFrontierB ? 'slot-frontier' : ''} ${isSlotBOverflow ? 'slot-overflow' : ''}" ${isFrontierB ? 'aria-current="true"' : ''} title="East Slot B (#${slotHi}): ${escapeHtml(metaB.tooltip)}${isSlotBOverflow ? ` (+${overflowFrames} overflow beyond 12-slot window)` : ''}">
            <span class="slot-main">
              <span class="slot-key">East B:</span>
              <span class="slot-val">${escapeHtml(metaB.readout)}</span>
            </span>
            ${isSlotBOverflow ? `<span class="overflow-badge" title="${overflowFrames} additional frame(s) beyond the 12-slot window">+${overflowFrames} overflow</span>` : ''}
          </div>
        </div>
      `;
    }

    // 10-Sector Control Stator Strip below R6 (innermost ring -> center stator)
    const activeSectorKey =
      statusInfo.label === 'Done'
        ? 'Return'
        : statusInfo.label === 'Rejected' || statusInfo.label === 'Exhausted'
          ? 'Error'
          : rawCtrlKind;
    const needleHemisphere =
      frontierChronoIdx < 0
        ? 'Center (0 frames)'
        : frontierChronoIdx % 2 === 0
          ? `West Slot A (R${activeRingIdx + 1}·A)`
          : `East Slot B (R${activeRingIdx + 1}·B)`;

    const sectorsHtml = CONTROL_STRIP_SECTORS.map((sec) => {
      const isSecActive = sec.key.toLowerCase() === activeSectorKey.toLowerCase();
      const toneCls = isSecActive
        ? statusInfo.label === 'Done'
          ? 'is-done'
          : statusInfo.label === 'Rejected' || statusInfo.label === 'Exhausted' || sec.key === 'Error'
            ? 'is-fault'
            : ''
        : '';
      return `<span class="control-sector-chip ${isSecActive ? 'active' : ''} ${toneCls}" ${isSecActive ? 'aria-current="step"' : ''} title="${escapeHtml(sec.title)}">${sec.badge}</span>`;
    }).join('');

    ringsHtml += `
      <div class="control-sector-strip" aria-label="10-Sector Control Stator">
        <div class="control-strip-top">
          <span>10-Sector Control Stator</span>
          <strong>Needle &rarr; ${escapeHtml(needleHemisphere)}</strong>
        </div>
        <div class="control-sector-grid">${sectorsHtml}</div>
      </div>
    `;

    if (ringsHtml !== state.lastRingsHtml) {
      state.lastRingsHtml = ringsHtml;
      els.sixRingsGauge.innerHTML = ringsHtml;
    }
  }

  // 6. Saved frames, top first
  els.frameStackCount.textContent = `${frames.length} of ${step.after.stackLimit} limit`;
  let framesHtml = '';
  if (frames.length === 0) {
    const rootEnvStr =
      activeBindings.length > 0
        ? `Active env: ${activeBindings.map((b) => `${b.name} = ${b.valueSummary}`).join(', ')}`
        : 'No saved frames.';
    framesHtml = `<p class="empty-note">${escapeHtml(rootEnvStr)}</p>`;
  } else {
    framesHtml = frames
      .map((fr, idx) => {
        const slotMeta = getFrameSlotMeta(fr);
        const chronoIdx = frames.length - 1 - idx;
        const ringNum = Math.floor(chronoIdx / 2) + 1;
        const slotChar = chronoIdx % 2 === 0 ? 'A' : 'B';
        const topClass = idx === 0 ? 'is-top' : '';
        const opLabel = formatAlgebraicText(fr.label);
        const envText = resolveFrameEnvText(frames, idx, activeBindings);
        const borrowedNote = fr.kind === 'Right' || fr.kind === 'Unary' ? ' (enclosing)' : '';
        const kindTag =
          ringNum <= 6
            ? `R${ringNum}·${slotChar} · ${slotMeta.family} · ${fr.kind}`
            : `+${chronoIdx - 11} overflow · ${slotMeta.family} · ${fr.kind}`;
        return `
          <div class="frame-item ${slotMeta.itemCls} ${topClass}">
            <div class="frame-item-top">
              <span class="frame-op">${escapeHtml(opLabel)}</span>
              <span class="frame-kind-tag">${escapeHtml(kindTag)}</span>
            </div>
            <div class="frame-env-row">Env${borrowedNote}: ${escapeHtml(envText)}</div>
          </div>
        `;
      })
      .join('');
  }
  if (framesHtml !== state.lastFramesHtml) {
    state.lastFramesHtml = framesHtml;
    els.frameStackList.innerHTML = framesHtml;
  }

  // 7. Active calls chain
  if (rec && els.recursionModePill && els.recursionCallsTrack) {
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
    els.recursionModePill.textContent = modeLabel;
    els.recursionModePill.className = `recursion-mode-pill ${
      rec.mode === 'UNWINDING' || rec.mode === 'RESOLVED'
        ? 'unwinding'
        : rec.mode === 'BASE CASE'
          ? 'base'
          : rec.mode === 'FAULT'
            ? 'fault'
            : ''
    }`;

    let callsHtml = '';
    if (rec.calls.length === 0) {
      callsHtml = `<span class="empty-note">Entry expression &rarr; ${escapeHtml(controlText)}</span>`;
    } else {
      callsHtml = rec.calls
        .map((c, idx) => {
          const retSuffix = c.status === 'returned' && c.returnValue ? ` = ${c.returnValue}` : '';
          const arrow = idx < rec.calls.length - 1 ? `<span class="rec-call-arrow">&rarr;</span>` : '';
          const stepTarget = c.status === 'returned' && c.returnStep !== null ? c.returnStep : c.invokeStep;
          return `<button type="button" class="rec-call-node ${c.status}" data-step="${stepTarget}" title="Jump to ${escapeHtml(c.label)}">${escapeHtml(c.label)}${escapeHtml(retSuffix)}</button>${arrow}`;
        })
        .join('');
    }
    if (callsHtml !== state.lastCallsHtml) {
      state.lastCallsHtml = callsHtml;
      els.recursionCallsTrack.innerHTML = callsHtml;
    }
  }
}

function updatePlayButton() {
  els.playPauseBtn.textContent = state.playing ? 'Pause' : 'Play';
}

// Delegated click listener for Active Calls chain buttons
if (els.recursionCallsTrack) {
  els.recursionCallsTrack.addEventListener('click', (ev) => {
    const node = ev.target.closest('.rec-call-node');
    if (!node) return;
    state.playing = false;
    updatePlayButton();
    state.playhead = Number(node.dataset.step || 0);
    els.stepScrubber.value = String(Math.floor(state.playhead));
    syncStepUI(Math.floor(state.playhead));
  });
}

// Event Listeners
els.compileRunBtn.addEventListener('click', () => executeCurrentSource(true));
els.sourceEditor.addEventListener('keydown', (ev) => {
  if ((ev.ctrlKey || ev.metaKey) && ev.key === 'Enter') {
    ev.preventDefault();
    executeCurrentSource(true);
  }
});
els.sourceEditor.addEventListener('input', () => {
  updateEditorLineNumbers(1);
  executeCurrentSource(false);
});
els.sourceEditor.addEventListener('scroll', () => {
  if (els.editorLineNumbers) {
    els.editorLineNumbers.scrollTop = els.sourceEditor.scrollTop;
  }
});

els.fuelSlider.addEventListener('input', () => {
  state.fuel = Number(els.fuelSlider.value);
  els.fuelValLabel.textContent = String(state.fuel);
  executeCurrentSource(false);
});

els.stackSlider.addEventListener('input', () => {
  state.stackLimit = Number(els.stackSlider.value);
  els.stackValLabel.textContent = String(state.stackLimit);
  executeCurrentSource(false);
});

els.playPauseBtn.addEventListener('click', () => {
  state.playing = !state.playing;
  if (state.playing && state.trace && state.playhead >= state.trace.steps.length - 1) {
    state.playhead = 0;
  }
  updatePlayButton();
});

els.stepPrevBtn.addEventListener('click', () => {
  state.playing = false;
  updatePlayButton();
  state.playhead = Math.max(0, Math.floor(state.playhead) - 1);
  els.stepScrubber.value = String(state.playhead);
  syncStepUI(Math.floor(state.playhead));
});

els.stepNextBtn.addEventListener('click', () => {
  state.playing = false;
  updatePlayButton();
  const maxStep = state.trace ? state.trace.steps.length - 1 : 0;
  state.playhead = Math.min(maxStep, Math.floor(state.playhead) + 1);
  els.stepScrubber.value = String(state.playhead);
  syncStepUI(Math.floor(state.playhead));
});

if (els.resetStepBtn) {
  els.resetStepBtn.addEventListener('click', () => {
    state.playing = false;
    updatePlayButton();
    state.playhead = 0;
    els.stepScrubber.value = '0';
    syncStepUI(0);
  });
}

els.stepScrubber.addEventListener('input', () => {
  state.playing = false;
  updatePlayButton();
  const nextStep = Math.round(Number(els.stepScrubber.value));
  state.playhead = nextStep;
  if (nextStep !== state.lastRenderedStepIdx) {
    syncStepUI(nextStep);
  }
});

els.speedSelect.addEventListener('change', () => {
  state.speed = Number(els.speedSelect.value || 1.0);
});

for (const btn of els.cameraLensBar.querySelectorAll('button')) {
  btn.addEventListener('click', () => {
    const preset = btn.dataset.preset;
    stage.setCameraPreset(preset);
    updateCameraLensButtons(preset);
  });
}

window.addEventListener('keydown', (ev) => {
  const active = document.activeElement;
  const tag = active?.tagName;
  if (
    tag === 'TEXTAREA' ||
    tag === 'INPUT' ||
    tag === 'SELECT' ||
    tag === 'BUTTON' ||
    tag === 'SUMMARY' ||
    active?.getAttribute?.('role') === 'button'
  ) {
    return;
  }
  if (ev.code === 'Space') {
    ev.preventDefault();
    els.playPauseBtn.click();
  } else if (ev.code === 'ArrowLeft') {
    ev.preventDefault();
    els.stepPrevBtn.click();
  } else if (ev.code === 'ArrowRight') {
    ev.preventDefault();
    els.stepNextBtn.click();
  } else if (ev.key >= '1' && ev.key <= '8') {
    const idx = Number(ev.key) - 1;
    if (PRESET_PROGRAMS[idx]) {
      ev.preventDefault();
      selectPreset(PRESET_PROGRAMS[idx]);
    }
  }
});

function getOrBuildStepState(stepIdx, stepFraction = 0, dt = 0.016) {
  const totalSteps = state.trace?.steps?.length || 0;
  const clampedIdx = Math.max(0, Math.min(totalSteps - 1, stepIdx));
  if (
    state.cachedStepState &&
    state.cachedTraceRef === state.trace &&
    state.cachedStepIdx === clampedIdx
  ) {
    const cached = state.cachedStepState;
    cached.stepFraction = stepFraction;
    cached.progress =
      totalSteps > 1
        ? Math.min(1, Math.max(0, (clampedIdx + stepFraction) / (totalSteps - 1)))
        : 1;
    cached.ambient = stage.ambient;
    cached.dt = dt;
    cached.presetId = state.activePresetId;
    cached.source = state.source;
    return cached;
  }

  const built = buildStepState({
    trace: state.trace,
    playhead: clampedIdx + stepFraction,
    ambient: stage.ambient,
    dt,
  });
  built.presetId = state.activePresetId;
  built.source = state.source;
  state.cachedStepState = built;
  state.cachedTraceRef = state.trace;
  state.cachedStepIdx = clampedIdx;
  return built;
}

// Animation Loop
function animate(now) {
  requestAnimationFrame(animate);
  const dt = Math.min(0.05, Math.max(0.001, (now - state.lastTick) / 1000));
  state.lastTick = now;

  if (state.trace && state.trace.steps.length > 0) {
    const maxStep = state.trace.steps.length - 1;
    if (state.playing) {
      state.playhead += dt * 3.2 * state.speed;
      if (state.playhead >= maxStep + 3.5) {
        state.playhead = 0;
      }
    }

    const clampedPlayhead = Math.min(maxStep, Math.max(0, state.playhead));
    const currentIdx = Math.floor(clampedPlayhead);
    const stepFraction = clampedPlayhead - currentIdx;
    const stepState = getOrBuildStepState(currentIdx, stepFraction, dt);

    if (currentIdx !== state.lastRenderedStepIdx) {
      syncStepUI(currentIdx, stepState);
    }

    stage.render(stepState);
  }
}

// Initialize Workbench
els.sourceEditor.value = state.source;
renderPresets();
executeCurrentSource(true);
requestAnimationFrame(animate);
