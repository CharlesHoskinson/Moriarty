// Pipeline page shell: timeline, playback loop, URL contract, camera rig, label layer,
// token carrier, handoff plates, stepped (reduced motion) mode. Scenes plug in through ctx.
import * as THREE from 'three/webgpu';
import * as theme from './theme.js';
import { PALETTE, clamp01, easeInOut, makeDigestToken, materials } from './theme.js';
import { LabelLayer } from './labels.js';

const SPACING = 40;         // distance between station centres along x
const SCENE_COUNT = 8;
const BASE_ASPECT = 1.6;    // aspect the shot spans are written for
const $ = (id) => document.getElementById(id);
const GATE_X = 5 * SPACING;   // world x of the boundary gate: centre of station 6
// Location of a beat: 'off' (your machine), 'cross' (the transaction), 'on' (Midnight). The scene's beat data sets it.
const WHERE = {
  off: { icon: 'computer', text: 'Off-chain: your machine', short: 'Off-chain' },
  cross: { icon: 'send', text: 'Crossing: the transaction', short: 'Crossing' },
  on: { icon: 'hub', text: 'On-chain: Midnight', short: 'On-chain' },
};
const whereOf = (si, b) => (b && WHERE[b.where] ? b.where : si < 6 ? 'off' : 'on');

// ------------------------------------------------------------------ camera shots
const rad = Math.PI / 180;
const shots = {
  // Three-quarter view from above the front edge. span = width of the bench to fit.
  establish(target = [0, 1, 0], span = 36) {
    const fov = 35;
    const d = span / 2 / (Math.tan(fov * rad / 2) * BASE_ASPECT);
    const dir = new THREE.Vector3(-0.3, 0.55, 0.78).normalize().multiplyScalar(d);
    return { pos: [target[0] + dir.x, target[1] + dir.y, target[2] + dir.z], target: [...target], fov };
  },
  // Close, nearly orthographic view square to the object. view: 'top' or 'front'.
  inspect(target = [0, 0, 0], span = 12, view = 'top') {
    const fov = 12;
    const d = span / 2 / (Math.tan(fov * rad / 2) * BASE_ASPECT);
    const off = view === 'front' ? [0, d * 0.04, d] : [0, d, d * 0.04];
    return { pos: [target[0] + off[0], target[1] + off[1], target[2] + off[2]], target: [...target], fov };
  },
  // Tracks one object along the bench. offset is the camera position relative to the target.
  follow(target = [0, 1, 0], offset = [-6, 6, 14]) {
    return { pos: [target[0] + offset[0], target[1] + offset[1], target[2] + offset[2]], target: [...target], fov: 30 };
  },
};

const lerpPose = (a, b, k) => ({
  pos: a.pos.map((v, i) => v + (b.pos[i] - v) * k),
  target: a.target.map((v, i) => v + (b.target[i] - v) * k),
  fov: a.fov + (b.fov - a.fov) * k,
});
const shiftPose = (p, dx) => ({ pos: [p.pos[0] + dx, p.pos[1], p.pos[2]], target: [p.target[0] + dx, p.target[1], p.target[2]], fov: p.fov });

// Pure function of time: pose at scene time t from keys [{t, pose, dur, cut}] starting from `start`.
function evalTrack(keys, t, start, cut) {
  let cur = start;
  for (const k of keys) {
    if (t < k.t) break;
    const dur = cut || k.cut ? 0 : (k.dur ?? 2);
    if (dur <= 0 || t >= k.t + dur) { cur = k.pose; continue; }
    return lerpPose(cur, k.pose, easeInOut((t - k.t) / dur));
  }
  return cur;
}

// ------------------------------------------------------------------ scene loading
function stubScene(i, err) {
  return {
    id: `scene${i}`, title: `Scene ${i}`,
    beats: [{ id: 'missing', duration: 12, narration: 'This scene does not load.', labels: [] }],
    build() {}, seek() {}, dispose() {}, _error: err,
  };
}

async function loadScenes() {
  return Promise.all(Array.from({ length: SCENE_COUNT }, async (_, k) => {
    const i = k + 1;
    try {
      const mod = await import(`./scenes/scene${i}.js`);
      const s = mod.default;
      if (!s || !Array.isArray(s.beats) || !s.beats.length) throw new Error('scene has no beats');
      return s;
    } catch (e) {
      console.error(`scene ${i} failed to load`, e);
      return stubScene(i, e);
    }
  }));
}

// ------------------------------------------------------------------ start
export async function start() {
  const q = new URLSearchParams(location.search);
  // WebGPU needs an adapter, not only navigator.gpu: without one the page runs on the WebGL2 backend by itself.
  let gpuOK = false;
  try { gpuOK = !!(navigator.gpu && await navigator.gpu.requestAdapter()); } catch (_) { gpuOK = false; }
  const forceWebGL = q.has('webgl') || !gpuOK;
  const seekMode = q.has('scene') || q.has('beat') || q.has('t');

  // Under the URL contract nothing may animate: no CSS transition or animation can be caught mid-flight.
  if (seekMode) document.documentElement.classList.add('pl-static');
  const stageEl = $('stage');
  const canvas = $('gl');
  const mqReduce = window.matchMedia('(prefers-reduced-motion: reduce)');
  let stepped = mqReduce.matches || q.has('stepped');
  document.documentElement.classList.toggle('pl-stepped', stepped);

  // ---------------- renderer, scene, lights, bench
  const renderer = new THREE.WebGPURenderer({ canvas, antialias: true, alpha: true, forceWebGL });
  renderer.setClearColor(0x000000, 0);
  renderer.setPixelRatio(Math.min(window.devicePixelRatio || 1, 2));
  renderer.outputColorSpace = THREE.SRGBColorSpace;
  renderer.toneMapping = THREE.ACESFilmicToneMapping;
  renderer.toneMappingExposure = 1.06;
  await renderer.init();

  const world = new THREE.Scene();
  // Ground: lighter dark surface on the prover side, deepest surface on the chain side.
  // The stage element carries the exact colour; the canvas is transparent.
  const paper = new THREE.Color(PALETTE.paper);
  const slate = new THREE.Color(PALETTE.wellSlate);
  const ground = paper.clone();
  world.fog = new THREE.Fog(paper.clone(), 140, 700);
  const camera = new THREE.PerspectiveCamera(35, 1.6, 0.1, 9000);
  world.add(camera);

  world.add(new THREE.HemisphereLight(0xfff3dc, 0x6b6556, 1.5));
  const key = new THREE.DirectionalLight(0xfff0dc, 2.4); key.position.set(-30, 60, 50); world.add(key);
  const fill = new THREE.DirectionalLight(0xcfe0ff, 0.8); fill.position.set(60, 25, 40); world.add(fill);
  const rim = new THREE.DirectionalLight(0xffffff, 0.6); rim.position.set(0, 30, -60); world.add(rim);

  const bench = new THREE.Group();
  world.add(bench);
  const BENCH_D = 14;
  for (let i = 0; i < SCENE_COUNT; i++) {
    const x = i * SPACING;
    const mk = (mat, w, dx) => {
      const m = new THREE.Mesh(new THREE.BoxGeometry(w, 0.6, BENCH_D), mat);
      m.position.set(x + dx, -0.3, 0);
      bench.add(m);
    };
    if (i < 5) mk(materials.paper(), SPACING - 0.2, 0);
    else if (i > 5) mk(materials.slate(), SPACING - 0.2, 0);
    else { mk(materials.paper(), SPACING / 2 - 0.1, -SPACING / 4); mk(materials.slate(), SPACING / 2 - 0.1, SPACING / 4); }
  }
  // Front rail in two zone colours, split at the gate; the gate itself (floor strip, two posts, lintel).
  const zoneMat = (hex) => new THREE.MeshStandardMaterial({ color: hex, roughness: 0.6, metalness: 0.1 });
  const zone = { off: zoneMat(PALETTE.zoneOff), on: zoneMat(PALETTE.zoneOn), offStrip: zoneMat(PALETTE.zoneOff), onStrip: zoneMat(PALETTE.zoneOn) };
  const railX0 = -SPACING / 2, railX1 = SPACING * (SCENE_COUNT - 0.5);
  const railPart = (x0, x1, mat) => {
    const m = new THREE.Mesh(new THREE.BoxGeometry(x1 - x0, 0.35, 0.45), mat);
    m.position.set((x0 + x1) / 2, 0.02, BENCH_D / 2 + 0.2);
    bench.add(m);
  };
  railPart(railX0, GATE_X, zone.off);
  railPart(GATE_X, railX1, zone.on);
  const gate = new THREE.Group();
  gate.position.set(GATE_X, 0, 0);
  const gbox = (w, h, d, x, y, z, mat) => { const m = new THREE.Mesh(new THREE.BoxGeometry(w, h, d), mat); m.position.set(x, y, z); gate.add(m); };
  gbox(0.2, 0.08, 14, -0.1, 0.04, 0, zone.offStrip);
  gbox(0.2, 0.08, 14, 0.1, 0.04, 0, zone.onStrip);
  const ti = materials.titanium();
  gbox(0.3, 7, 0.3, 0, 3.5, -7, ti);
  gbox(0.3, 7, 0.3, 0, 3.5, 7, ti);
  gbox(0.3, 0.3, 14.3, 0, 7, 0, ti);
  bench.add(gate);
  const gateLabelPos = [new THREE.Vector3(GATE_X - 2, 7, 7), new THREE.Vector3(GATE_X + 2, 7, 7)];
  const glowOf = { off: [zone.off], gate: [zone.offStrip, zone.onStrip], on: [zone.on] };
  const glowAmt = { off: 0, gate: 0, on: 0 };
  const glowHex = { off: PALETTE.zoneOff, gate: PALETTE.gold, on: PALETTE.zoneOn };
  function applyGlow() {
    for (const k of Object.keys(glowOf)) {
      for (const m of glowOf[k]) {
        m.emissive.setHex(glowHex[k]);
        m.emissiveIntensity = 0.5 * glowAmt[k];
      }
    }
  }
  // Handoff plates at every station boundary.
  const plateX = (k) => -SPACING / 2 + k * SPACING;
  for (let k = 0; k <= SCENE_COUNT; k++) {
    const edge = new THREE.Mesh(new THREE.BoxGeometry(4.3, 0.12, 4.3), materials.gold());
    edge.position.set(plateX(k), 0.06, 0);
    const plate = new THREE.Mesh(new THREE.BoxGeometry(4, 0.2, 4), materials.titanium());
    plate.position.set(plateX(k), 0.1, 0);
    bench.add(edge, plate);
  }

  // ---------------- scenes
  const defs = await loadScenes();
  const labels = new LabelLayer($('labels'));

  // Timeline: scenes concatenate their beats.
  const tl = { scenes: [], total: 0 };
  defs.forEach((def, i) => {
    let t = 0;
    const beats = def.beats.map((b, bi) => {
      const e = { index: bi, def: b, local: t, dur: Math.max(0.1, +b.duration || 12) };
      t += e.dur;
      return e;
    });
    tl.scenes.push({ idx: i, def, start: tl.total, dur: t, beats });
    tl.total += t;
  });

  // Token carrier: four tokens fixed to the camera, constant size on screen.
  const carrierGroup = new THREE.Group();
  camera.add(carrierGroup);
  const tray = new THREE.Mesh(new THREE.BoxGeometry(1, 0.14, 0.05), materials.titanium());
  tray.material.depthTest = false; tray.renderOrder = 997;
  carrierGroup.add(tray);
  const CARRIER = [
    { name: 'prog_d', family: 'program', content: 'prog:main' },
    { name: 'input_d', family: 'input', content: 'input:main' },
    { name: 'outcome_d', family: 'outcome', content: 'outcome:main' },
    { name: 'n_steps', family: 'state', content: 'n_steps:main' },
  ].map((c) => {
    const tok = makeDigestToken({ family: c.family, content: c.content, size: 1, overlay: true });
    tok.userData.baseContent = c.content;
    carrierGroup.add(tok);
    return { ...c, tok, amount: 0, auto: 0 };
  });
  const carrierByName = Object.fromEntries(CARRIER.map((c) => [c.name, c]));
  // Carrier layout for this frame: side of the stage ('left' default, 'right') and whether the token names show.
  // A beat declares it as `beat.carrier = { side, labels }`; a scene may also call ctx.carrier.layout in seek.
  const carrierLayout = { side: 'left', labels: true };
  const carrierDefault = (si) => ({ prog_d: si >= 1 ? 1 : 0, input_d: si >= 1 ? 1 : 0, outcome_d: si >= 5 ? 1 : 0, n_steps: si >= 5 ? 1 : 0 });

  // Per-scene state and ctx.
  let pendingCode = null;
  const states = tl.scenes.map((s, i) => {
    const group = new THREE.Group();
    group.position.x = i * SPACING;
    world.add(group);
    const st = { i, s, group, live: false, wide: false, fog: null, track: [], anchors: new Map(), lastT: null, ok: true };
    const toWorld = (v) => new THREE.Vector3(v[0] ?? v.x, v[1] ?? v.y, v[2] ?? v.z).add(group.position);
    st.toWorld = toWorld;
    st.ctx = {
      index: i + 1, scene: group, theme, renderer, SPACING,
      reducedMotion: stepped,
      duration: s.dur,
      plates: { in: new THREE.Vector3(-SPACING / 2, 0.2, 0), out: new THREE.Vector3(SPACING / 2, 0.2, 0) },
      beatAt: (t) => {
        let b = s.beats[0];
        for (const e of s.beats) if (t >= e.local) b = e;
        return { index: b.index, id: b.def.id, t: Math.min(Math.max(t - b.local, 0), b.dur), start: b.local, duration: b.dur };
      },
      camera: {
        shots,
        // keys: [{ t, pose, dur = 2, cut = false }], t in scene seconds, scene-local coordinates.
        track: (keys) => { st.track = keys.slice().sort((a, b) => a.t - b.t); },
      },
      labels: {
        // anchor: scene-local [x,y,z], a Vector3 in scene-local space, or an Object3D.
        show: (id, html, anchor, opts = {}) => {
          if (!st.live) return;
          let p;
          if (anchor && anchor.isObject3D) { p = new THREE.Vector3(); anchor.updateWorldMatrix(true, false); anchor.getWorldPosition(p); if (opts.dy) p.y += opts.dy; }
          else { p = toWorld(anchor); if (opts.dy) p.y += opts.dy; }
          labels.show(`s${i}:${id}`, html, p, opts);
        },
        // Register a named anchor for beat.labels entries.
        anchor: (name, obj) => { st.anchors.set(name, obj); },
      },
      carrier: {
        // amount 0..1 (presence), optional content string for the dot face.
        show: (name, amount = 1, content) => {
          if (!st.live) return;
          const c = carrierByName[name];
          if (!c) return;
          c.amount = clamp01(amount);
          if (content !== undefined) { c.shown = content; c.tok.userData.setContent(content); }
        },
        // Reserve the lower-left of the stage for scene content: { side: 'left' | 'right', labels: boolean }.
        layout: (o = {}) => {
          if (!st.live) return;
          if (o.side === 'left' || o.side === 'right') carrierLayout.side = o.side;
          if (o.labels !== undefined) carrierLayout.labels = !!o.labels;
        },
      },
      // Call inside seek: light a zone for this frame, amount 0..1 (emissive 0.5 at 1). zone: 'off' (off-chain rail),
      // 'gate' (the gate's floor strip, gold) or 'on' (on-chain rail). Reset every frame.
      zone: { glow: (name, amount = 1) => { if (st.live && name in glowAmt) glowAmt[name] = clamp01(amount); } },
      // Call inside seek: fog range for this frame (reset each frame; default 140 to 700, wide 800 to 9000).
      setFog: (near, far) => { if (st.live) st.fog = [near, far]; },
      // Code card over the stage. show(lines, { highlight: [1-based line numbers], start = 1, title }) inside seek.
      code: {
        show: (lines, opts = {}) => { if (st.live) pendingCode = { lines, ...opts }; },
        hide: () => { if (st.live) pendingCode = null; },
      },
      // Call inside seek: draw every station (the wide shot of scene 8).
      showAll: (flag = true) => { if (st.live) st.wide = !!flag; },
    };
    return st;
  });

  for (const st of states) {
    try { await st.s.def.build(st.ctx); }
    catch (e) { console.error(`scene ${st.i + 1} build failed`, e); st.ok = false; }
  }

  // ---------------- timeline lookup
  function locate(T) {
    T = Math.min(Math.max(T, 0), tl.total);
    let sc = tl.scenes[0];
    for (const s of tl.scenes) if (T >= s.start) sc = s;
    const ts = Math.min(T - sc.start, sc.dur);
    let b = sc.beats[0];
    for (const e of sc.beats) if (ts >= e.local) b = e;
    return { si: sc.idx, bi: b.index, tScene: ts, tBeat: Math.min(ts - b.local, b.dur), sc, b };
  }

  const sceneEndPose = (si) => {
    const st = states[si];
    if (!st.track.length) return shiftPose(shots.establish([0, 1, 0], 34), si * SPACING);
    return shiftPose(st.track[st.track.length - 1].pose, si * SPACING);
  };
  function cameraPose(si, tScene, cut) {
    const st = states[si];
    const dx = si * SPACING;
    if (!st.track.length) return sceneEndPose(si);
    const keys = st.track.map((k) => ({ ...k, pose: shiftPose(k.pose, dx) }));
    const first = shiftPose(st.track[0].pose, dx);
    const start = si > 0 ? sceneEndPose(si - 1) : first;
    return evalTrack(keys, tScene, start, cut);
  }

  // ---------------- per-frame application
  let stageW = 1, stageH = 1;
  const tmpV = new THREE.Vector3();
  let carrierStagger = false, carrierUnit = 0, narrow = false;
  function layoutCarrier() {
    const dist = 6;
    const halfH = dist * Math.tan(camera.fov * rad / 2);
    const halfW = halfH * camera.aspect;
    const s = halfH * 0.1;
    narrow = stageW < 560;
    // On a phone the four tokens spread over the whole stage width so the four names fit side by side.
    const slot = narrow ? (halfW * 2 - s * 3.2) / 4 : Math.min(s * 2.5, (halfW * 2 - s * 2) / 4);
    carrierStagger = false;
    carrierUnit = s;
    const right = carrierLayout.side === 'right';
    const x0 = right ? halfW - s * 1.6 - slot * 4 : -halfW + s * 1.6;
    let any = 0;
    CARRIER.forEach((c, k) => {
      c.tok.visible = c.amount > 0.001;
      const sc = s * 2 * (0.4 + 0.6 * easeInOut(c.amount));
      c.tok.scale.setScalar(sc);
      c.tok.position.set(x0 + slot * (k + 0.5), -halfH + s * 2.0, -dist);
      any = Math.max(any, c.amount);
    });
    tray.visible = any > 0.001;
    tray.scale.set(slot * 4 + s * 0.6, s * 1.6, 1);
    tray.position.set(x0 + slot * 2, -halfH + s * 2.0 - s * 1.25, -dist);
    carrierGroup.updateMatrixWorld(true);
  }

  // Code card: surface-container-high card with line numbers and highlighted lines.
  const codeEl = $('codeCard');
  const codeList = $('codeList');
  const codeTitle = $('codeTitle');
  let codeSig = '';
  const LINE_PX = 18;
  function renderCode(c) {
    if (!c) { codeEl.hidden = true; return; }
    const start = c.start ?? 1;
    const sig = JSON.stringify([c.lines, start, c.title || '']);
    if (sig !== codeSig) {
      codeSig = sig;
      codeList.innerHTML = '';
      c.lines.forEach((text, k) => {
        const li = document.createElement('li');
        const n = document.createElement('span'); n.className = 'n'; n.textContent = String(start + k);
        const t = document.createElement('span'); t.className = 't'; t.textContent = text;
        li.append(n, t);
        codeList.appendChild(li);
      });
      codeTitle.textContent = c.title || '';
      codeTitle.hidden = !c.title;
    }
    const hl = new Set(c.highlight || []);
    let first = -1;
    [...codeList.children].forEach((li, k) => {
      const on = hl.has(start + k);
      if (on && first < 0) first = k;
      li.classList.toggle('hl', on);
    });
    codeEl.hidden = false;
    codeList.parentElement.scrollTop = first < 0 ? 0 : Math.max(0, (first - 2) * LINE_PX);
  }

  // Level of detail for the wide pull-back (ctx.showAll): after the camera is set, every mesh of the other
  // stations whose projected radius is under LOD_PX pixels is hidden for this frame and restored after the render.
  const LOD_PX = 2.4;
  const lodHidden = [];
  const lodSphere = new THREE.Sphere();
  function applyLOD(skipGroup) {
    const halfTan = Math.tan(camera.fov * rad / 2);
    const k = (stageH / 2) / halfTan;
    for (const st of states) {
      if (st.group === skipGroup || !st.group.visible) continue;
      st.group.updateMatrixWorld(true);
      st.group.traverse((o) => {
        if (!o.isMesh || !o.visible) return;
        const g = o.geometry;
        if (!g.boundingSphere) g.computeBoundingSphere();
        lodSphere.copy(g.boundingSphere).applyMatrix4(o.matrixWorld);
        const d = Math.max(0.1, lodSphere.center.distanceTo(camera.position));
        if (lodSphere.radius / d * k < LOD_PX) { o.visible = false; lodHidden.push(o); }
      });
    }
  }
  function restoreLOD() { for (const o of lodHidden) o.visible = true; lodHidden.length = 0; }

  // ---------------- location chip, inventory card
  const whereChip = $('whereChip'), whereIcon = $('whereIcon'), whereText = $('whereText');
  const whereCard = $('whereCard');
  let shownWhere = '', cardSig = '', cardEls = [];
  function setWhere(w) {
    if (w === shownWhere) return;
    const first = shownWhere === '';
    shownWhere = w;
    const m = WHERE[w];
    whereChip.dataset.where = w;
    whereIcon.textContent = m.icon;
    whereText.textContent = m.text;
    if (!first && !seekMode && !stepped) { whereChip.classList.remove('swap'); void whereChip.offsetWidth; whereChip.classList.add('swap'); }
  }
  function renderCard(inv, tBeat) {
    if (!inv) { whereCard.hidden = true; return; }
    const sig = JSON.stringify(inv);
    if (sig !== cardSig) {
      cardSig = sig;
      whereCard.innerHTML = '';
      cardEls = [];
      const cols = document.createElement('div'); cols.className = 'pl-wcols';
      for (const c of inv.columns) {
        const col = document.createElement('div'); col.className = 'pl-wcol';
        const chip = document.createElement('div'); chip.className = 'pl-where'; chip.dataset.where = c.where;
        chip.innerHTML = `<span class="ms" aria-hidden="true">${WHERE[c.where].icon}</span><span class="pl-wt"></span>`;
        chip.lastChild.textContent = WHERE[c.where].text;
        const ul = document.createElement('ul');
        for (const it of c.items) { const li = document.createElement('li'); li.innerHTML = it; ul.appendChild(li); }
        col.append(chip, ul);
        cols.appendChild(col);
        cardEls.push({ el: col, from: c.from ?? 0 });
      }
      whereCard.appendChild(cols);
      if (inv.compare) {
        const cmp = document.createElement('div'); cmp.className = 'pl-wcmp';
        for (const cell of inv.compare.cells) {
          const d = document.createElement('div');
          const h3 = document.createElement('h3'); h3.textContent = cell.title;
          const p = document.createElement('p'); p.innerHTML = cell.text;
          d.append(h3, p); cmp.appendChild(d);
        }
        whereCard.appendChild(cmp);
        cardEls.push({ el: cmp, from: inv.compare.from ?? 0 });
      }
    }
    whereCard.hidden = false;
    for (const c of cardEls) c.el.classList.toggle('on', stepped || tBeat >= c.from);
  }

  let curSi = -1;
  function applyTime(T) {
    const loc = locate(T);
    const { si, sc } = loc;
    // Stepped mode shows the hold pose at the end of the beat.
    // (a hair before the end: at exactly the end, beatAt would return the next beat)
    const tScene = stepped ? loc.b.local + loc.b.dur - 1e-3 : loc.tScene;
    const tBeat = stepped ? loc.b.dur - 1e-3 : loc.tBeat;
    curSi = si;
    states.forEach((st, i) => { st.group.visible = i === si; st.wide = false; st.fog = null; });
    pendingCode = null;
    const st = states[si];

    labels.begin();
    glowAmt.off = glowAmt.gate = glowAmt.on = 0;
    const def = carrierDefault(si);
    CARRIER.forEach((c) => { c.amount = def[c.name]; c.shown = c.content; c.tok.userData.setContent(c.content); });
    st.ctx.reducedMotion = stepped;
    st.live = true;
    carrierLayout.side = loc.b.def.carrier?.side || 'left';
    carrierLayout.labels = loc.b.def.carrier?.labels !== false;
    if (st.ok) {
      try { sc.def.seek(tScene); } catch (e) { console.error(`scene ${si + 1} seek failed`, e); }
    }
    // Declared beat labels.
    for (const [li, l] of (loc.b.def.labels || []).entries()) {
      if (l.from != null && tBeat < l.from) continue;
      if (l.to != null && tBeat > l.to) continue;
      let p;
      if (l.anchor && st.anchors.has(l.anchor)) {
        const o = st.anchors.get(l.anchor); p = new THREE.Vector3(); o.updateWorldMatrix(true, false); o.getWorldPosition(p);
      } else if (l.at) p = st.toWorld(l.at);
      else continue;
      if (l.dy) p.y += l.dy;
      labels.show(`b${si}:${loc.bi}:${l.id ?? li}`, l.text, p, { kind: l.kind });
    }
    st.live = false;
    renderCode(pendingCode);
    setWhere(whereOf(si, loc.b.def));
    renderCard(loc.b.def.inventory, tBeat);
    applyGlow();

    if (st.wide) {
      states.forEach((o, i) => {
        if (i === si) return;
        o.group.visible = true;
        if (o.ok) { try { o.s.def.seek(o.s.dur); } catch (e) { console.error(e); } }
      });
    }

    // Ground colour: paper to slate across the crossing (scene 6).
    const mix = si < 5 ? 0 : si > 5 ? 1 : easeInOut(tScene / sc.dur);
    ground.copy(paper).lerp(slate, mix);
    world.fog.color.copy(ground);
    const fg = st.fog || (st.wide ? [800, 9000] : [140, 700]);
    world.fog.near = fg[0]; world.fog.far = fg[1];
    stageEl.style.backgroundColor = `#${ground.getHexString()}`;

    // Camera.
    const pose = cameraPose(si, tScene, stepped);
    camera.position.set(...pose.pos);
    camera.up.set(0, 1, 0);
    camera.lookAt(...pose.target);
    const aspect = stageW / stageH;
    camera.aspect = aspect;
    camera.fov = 2 * Math.atan(Math.tan(pose.fov * rad / 2) * Math.max(1, BASE_ASPECT / aspect)) / rad;
    camera.updateProjectionMatrix();
    camera.updateMatrixWorld(true);
    if (st.wide) applyLOD(st.group);

    // Carrier labels.
    layoutCarrier();
    for (const [ci, c] of CARRIER.entries()) {
      if (!c.tok.visible || c.amount < 0.999) continue;
      c.tok.userData.anchor.getWorldPosition(tmpV);
      if (!carrierLayout.labels) continue;
      labels.show(`c:${c.name}`, `<code>${c.name}</code>`, tmpV, { kind: c.family, carrier: true, compact: true });
    }
    // Zone labels on the front post of the gate, whenever the gate is in frame. They do not count toward the six.
    labels.show('z:off', 'Off-chain', gateLabelPos[0], { kind: 'zoneoff', carrier: true });
    labels.show('z:on', 'On-chain', gateLabelPos[1], { kind: 'zoneon', carrier: true });
    labels.fixed.length = 0;
    for (const e of [$('stageBadge'), whereChip]) {
      const r = e.getBoundingClientRect(), sr = stageEl.getBoundingClientRect();
      labels.fixed.push({ x0: r.left - sr.left - 4, y0: r.top - sr.top - 4, x1: r.right - sr.left + 4, y1: r.bottom - sr.top + 4 });
    }
    if (!whereCard.hidden) labels.fixed.push({ x0: 0, y0: stageH * 0.38, x1: stageW, y1: stageH });
    labels.flush(camera, stageW, stageH);
    renderer.render(world, camera);
    restoreLOD();
    return loc;
  }

  // ---------------- UI
  const sceneList = $('sceneList');
  const narr = $('narration');
  const scrub = $('scrub');
  const playBtn = $('playBtn');
  const playIcon = $('playIcon');
  const timeEl = $('timeText');
  const badge = $('stageBadge');
  const fmt = (s) => `${Math.floor(s / 60)}:${String(Math.floor(s % 60)).padStart(2, '0')}`;
  const SCENE_ICONS = ['description', 'directions_run', 'memory', 'account_tree', 'verified', 'send', 'gavel', 'hub'];
  const humanize = (id) => { const t = String(id || '').replace(/[-_]+/g, ' ').trim(); return t.charAt(0).toUpperCase() + t.slice(1); };

  scrub.max = String(tl.total);
  scrub.step = '0.1';
  const ticks = $('ticks');
  for (const s of tl.scenes) {
    for (const b of s.beats) {
      if (s.idx === 0 && b.index === 0) continue;
      const d = document.createElement('i');
      d.style.setProperty('--p', String(((s.start + b.local) / tl.total)));
      if (b.index === 0) d.className = 'scene';
      ticks.appendChild(d);
    }
  }
  // Scrubber band: the location of every beat along the timeline, in three runs.
  {
    const col = { off: 'var(--md-sys-color-secondary-container)', cross: 'var(--md-sys-color-primary-container)', on: 'var(--md-sys-color-tertiary-container)' };
    const runs = [];
    for (const s of tl.scenes) for (const b of s.beats) {
      const w = whereOf(s.idx, b.def), a = (s.start + b.local) / tl.total, z = (s.start + b.local + b.dur) / tl.total;
      if (runs.length && runs[runs.length - 1].w === w) runs[runs.length - 1].z = z; else runs.push({ w, a, z });
    }
    $('whereBand').style.background = `linear-gradient(to right, ${runs.map((r) => `${col[r.w]} ${(r.a * 100).toFixed(3)}% ${(r.z * 100).toFixed(3)}%`).join(', ')})`;
  }
  const sceneBtns = tl.scenes.map((s) => {
    const li = document.createElement('li');
    const b = document.createElement('button');
    b.type = 'button';
    b.className = 'pl-dest pl-state';
    const short = s.def.title.replace(/^The\s+/i, '');
    b.innerHTML = `<span class="pl-ind"><span class="ms" aria-hidden="true">${SCENE_ICONS[s.idx] || 'circle'}</span></span><span class="pl-lab"></span>`;
    b.lastChild.textContent = short.charAt(0).toUpperCase() + short.slice(1);
    b.setAttribute('aria-label', `Scene ${s.idx + 1}: ${s.def.title}`);
    b.addEventListener('click', () => { setTime(s.start); });
    li.appendChild(b);
    if (s.idx === 0 || s.idx === 6) {
      const sect = document.createElement('li');
      sect.className = 'pl-sect';
      sect.textContent = s.idx === 0 ? 'Off-chain' : 'On-chain';
      sceneList.appendChild(sect);
    }
    sceneList.appendChild(li);
    return b;
  });

  let T = 0, playing = false, lastTs = 0, dirty = true;
  let shownBeat = '';

  function updateUI(loc) {
    const key = `${loc.si}:${loc.bi}`;
    if (key !== shownBeat) {
      shownBeat = key;
      const b = loc.b.def;
      narr.innerHTML = '';
      const h = document.createElement('h2'); h.textContent = b.title || humanize(b.id); narr.appendChild(h);
      const meta = document.createElement('p'); meta.className = 'pl-meta';
      meta.textContent = `${WHERE[whereOf(loc.si, b)].short}. ${loc.sc.def.title}, scene ${loc.si + 1} of ${SCENE_COUNT}, beat ${loc.bi + 1} of ${loc.sc.beats.length}`;
      narr.appendChild(meta);
      for (const para of String(b.narration || '').split(/\n\s*\n/)) {
        const p = document.createElement('p'); p.className = 'pl-para'; p.innerHTML = para.trim(); narr.appendChild(p);
      }
      sceneBtns.forEach((btn, i) => { btn.classList.toggle('on', i === loc.si); if (i === loc.si && sceneList.scrollWidth > sceneList.clientWidth) sceneList.scrollLeft = Math.max(0, btn.parentElement.offsetLeft - 16); btn.setAttribute('aria-current', i === loc.si ? 'step' : 'false'); });
      badge.textContent = `Scene ${loc.si + 1} of ${SCENE_COUNT}: ${loc.sc.def.title}`;
      canvas.setAttribute('aria-label', `Scene ${loc.si + 1}, ${loc.sc.def.title}, beat ${loc.bi + 1}`);
      narr.scrollTop = 0;
    }
    scrub.value = String(T);
    scrub.style.setProperty('--fill', `${(T / tl.total) * 100}%`);
    timeEl.textContent = `${fmt(T)} / ${fmt(tl.total)}`;
    playBtn.setAttribute('aria-label', playing ? 'Pause' : (T >= tl.total - 0.001 ? 'Replay' : 'Play'));
    const ic = playing ? 'pause' : (T >= tl.total - 0.001 ? 'replay' : 'play_arrow');
    if (playIcon.textContent !== ic) playIcon.textContent = ic;
  }

  function setTime(t, { pause = false } = {}) {
    T = Math.min(Math.max(t, 0), tl.total);
    if (pause) playing = false;
    dirty = true;
  }
  function setPlaying(p) {
    if (p && T >= tl.total - 0.001) T = 0;
    playing = p; lastTs = performance.now(); dirty = true;
  }
  const beatStarts = tl.scenes.flatMap((s) => s.beats.map((b) => s.start + b.local));
  const sceneStarts = tl.scenes.map((s) => s.start);
  const EPS = 1.5;
  function step(list, dir) {
    if (dir > 0) { const n = list.find((x) => x > T + 0.001); setTime(n ?? tl.total); }
    else { const prev = list.filter((x) => x < T - EPS); setTime(prev.length ? prev[prev.length - 1] : 0); }
  }
  $('prevScene').addEventListener('click', () => step(sceneStarts, -1));
  $('nextScene').addEventListener('click', () => step(sceneStarts, 1));
  $('prevBeat').addEventListener('click', () => step(beatStarts, -1));
  $('nextBeat').addEventListener('click', () => step(beatStarts, 1));
  playBtn.addEventListener('click', () => setPlaying(!playing));
  scrub.addEventListener('input', () => setTime(parseFloat(scrub.value)));
  document.addEventListener('keydown', (e) => {
    if (e.ctrlKey || e.metaKey || e.altKey) return;
    const map = { ' ': () => setPlaying(!playing), ArrowRight: () => step(beatStarts, 1), ArrowLeft: () => step(beatStarts, -1),
      ArrowDown: () => step(sceneStarts, 1), ArrowUp: () => step(sceneStarts, -1), PageDown: () => step(sceneStarts, 1), PageUp: () => step(sceneStarts, -1),
      Home: () => setTime(0), End: () => setTime(tl.total) };
    const fn = map[e.key];
    if (!fn) return;
    e.preventDefault();
    if (e.target && e.target.matches && e.target.matches('button, input')) e.target.blur();
    fn();
  });
  mqReduce.addEventListener?.('change', (e) => { stepped = e.matches || q.has('stepped'); document.documentElement.classList.toggle('pl-stepped', stepped); dirty = true; });

  new ResizeObserver(() => {
    const r = stageEl.getBoundingClientRect();
    stageW = Math.max(2, Math.round(r.width)); stageH = Math.max(2, Math.round(r.height));
    renderer.setSize(stageW, stageH, false);
    dirty = true;
  }).observe(stageEl);
  const r0 = stageEl.getBoundingClientRect();
  stageW = Math.max(2, Math.round(r0.width)); stageH = Math.max(2, Math.round(r0.height));
  renderer.setSize(stageW, stageH, false);

  // ---------------- initial time from the URL contract
  if (seekMode) {
    const sN = Math.min(Math.max(parseInt(q.get('scene') || '1', 10) || 1, 1), SCENE_COUNT);
    const sc = tl.scenes[sN - 1];
    const bN = Math.min(Math.max(parseInt(q.get('beat') || '0', 10) || 0, 0), sc.beats.length - 1);
    const b = sc.beats[bN];
    const t = Math.min(Math.max(parseFloat(q.get('t') || '0') || 0, 0), b.dur);
    T = sc.start + b.local + t;
    playing = false;
  } else {
    playing = true; lastTs = performance.now();
  }

  // ---------------- loop
  let lastKey = '';
  function frame(ts) {
    requestAnimationFrame(frame);
    if (playing) {
      const dt = Math.min(0.1, Math.max(0, (ts - lastTs) / 1000));
      lastTs = ts;
      T += dt;
      if (T >= tl.total) { T = tl.total; playing = false; dirty = true; }
      const loc = locate(T);
      const k = `${loc.si}:${loc.bi}`;
      if (!stepped || k !== lastKey) dirty = true;
      lastKey = k;
    }
    if (dirty) {
      dirty = false;
      const loc = applyTime(T);
      updateUI(loc);
    }
  }

  const loc0 = applyTime(T);
  updateUI(loc0);
  try { await renderer.compileAsync(world, camera); } catch (_) { /* compile is an optimisation */ }
  applyTime(T);
  // Fonts come from Google Fonts. Under the URL contract the page waits for every face it draws with, so a capture
  // never catches a fallback face or a half-loaded icon font; interactive use waits at most 2.5 s.
  const faces = ['400 14px "Roboto Flex"', '500 14px "Roboto Flex"', '500 12px "Roboto Mono"', '400 24px "Material Symbols Rounded"'];
  const fontsDone = document.fonts
    ? Promise.all(faces.map((f) => document.fonts.load(f).catch(() => null)))
      .then(() => (seekMode ? Promise.all([...document.fonts].map((f) => f.load().catch(() => null))) : null))
      .then(() => document.fonts.ready)
    : Promise.resolve();
  await Promise.race([fontsDone, new Promise((r) => setTimeout(r, seekMode ? 20000 : 2500))]);
  applyTime(T);
  requestAnimationFrame(frame);
  await new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)));
  window.__pipeline = { get time() { return T; }, setTime, setPlaying, timeline: tl, locate, states, applyTime, carrier: () => CARRIER.map((c) => ({ name: c.name, amount: +c.amount.toFixed(3), content: c.shown })) };
  // Probe for checks the capture tool cannot run: render cost, seek cost, seek-order independence.
  const snap = document.createElement('canvas');
  const hashPixels = () => {
    snap.width = canvas.width; snap.height = canvas.height;
    const g = snap.getContext('2d', { willReadFrequently: true });
    g.drawImage(canvas, 0, 0);
    const px = new Uint32Array(g.getImageData(0, 0, snap.width, snap.height).data.buffer);
    let h = 2166136261 >>> 0;
    for (let i = 0; i < px.length; i++) { h ^= px[i]; h = Math.imul(h, 16777619) >>> 0; }
    return h.toString(16);
  };
  window.__probe = async () => {
    const T0 = T;
    const loc = locate(T0);
    const st = states[loc.si];
    const sc = loc.sc;
    applyTime(T0);
    const info = renderer.info.render;
    const drawCalls = info.drawCalls ?? info.calls;
    const triangles = info.triangles;
    // 600 seek calls spread over the scene, timed in 30 batches of 20 (the clock is coarse); median batch mean.
    const N = 600, B = 20, ms = [];
    labels.begin();
    st.live = true;
    for (let k = 0; k < N / B; k++) {
      const a = performance.now();
      for (let i = 0; i < B; i++) {
        try { sc.def.seek((sc.dur * (k * B + i + 0.5)) / N); } catch (e) { console.error(e); }
      }
      ms.push((performance.now() - a) / B);
    }
    st.live = false; st.wide = false; st.fog = null;
    ms.sort((x, y) => x - y);
    applyTime(T0);
    const hashA = hashPixels();
    applyTime(sc.start + sc.dur);
    applyTime(sc.start + sc.dur * 0.37);
    applyTime(sc.start);
    applyTime(T0);
    const hashB = hashPixels();
    applyTime(T0);
    const geos = new Set(), mats = new Set();
    st.group.traverse((o) => {
      if (o.geometry) geos.add(o.geometry);
      if (o.material) (Array.isArray(o.material) ? o.material : [o.material]).forEach((m) => mats.add(m));
    });
    return { scene: loc.si + 1, beat: loc.bi, t: +loc.tBeat.toFixed(2), drawCalls, triangles, geometries: geos.size, materials: mats.size, seekMs: +ms[ms.length >> 1].toFixed(4), seekOrderEqual: hashA === hashB, hashA, hashB };
  };
  window.__ready = true;

  addEventListener('pagehide', () => { for (const st of states) { try { st.s.def.dispose(); } catch (_) { /* ignore */ } } });
}
