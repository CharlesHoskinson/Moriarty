import * as THREE from 'three/webgpu';
import { PALETTE, materials, phase, ramp, lerp } from '../theme.js';
import { makeProofSlab, makeStateToken } from '../kit.js';
import { Rig, Track, makeTok, shareToken, disposeShared } from './scene5/rig.js';
import { BEATS } from './scene5/beats.js';

// Scene 5: the decider. Timeline: every object is a track over scene time; beat b starts at ST[b].
const ST = [0, 21, 43, 67, 89, 108, 130, 154, 174, 198, 220, 244, 267, 291];
const T = (b, s) => ST[b] + s;
const HALF = Math.PI / 2;
const BASE = 0.3; // top of the decider housing floor

let ctx = null, rig = null;
let rootP, rootSlab, rootMats, rootPlug, sealP, sealSlab, postP, postTok, hashP, hashMat;
let runP, glow, rootVerify, rootDim, plugY, segs, frames, extraDisposables;
const sc1 = [0];

const val = (tr, t) => tr.at(t, sc1)[0];

// A track-driven scalar.
const scalar = (v) => new Track([v]);

// Make a prop that is a single token lying face up (scale = token size).
function tokP(family, content, size, x, y, z, shown = false) {
  const p = rig.prop([x, y, z, shown ? size : 0]);
  const tok = makeTok(family, content);
  tok.rotation.x = -HALF;
  p.node.add(tok);
  p.size = size;
  p.tok = tok;
  return p;
}

function cellP(pool, x, y, z, s = 0, bars = 0) {
  const p = rig.prop([x, y, z, s]);
  p.box(pool, 0, 0, 0, 0.4, 0.4, 0.4);
  for (let i = 0; i < bars; i++) p.box('ink', 0, 0.21, (i - (bars - 1) / 2) * 0.12, 0.2, 0.02, 0.06);
  return p;
}

const appear = (p, b, s, dur = 0.4, size = 1) => p.tr.go(T(b, s), T(b, s + dur), [[undefined, undefined, undefined, size]]);
const leave = (p, b, s, dur = 0.4) => p.tr.go(T(b, s), T(b, s + dur), [[undefined, undefined, undefined, 0]]);
const grow = (tr, b, s0, s1) => tr.go(T(b, s0), T(b, s1), [[1]]);
const vanish = (tr, b, s0, s1) => tr.go(T(b, s0), T(b, s1), [[0]]);

let geoShared = null, matShared = null;
function sharedGeos() {
  if (geoShared) return;
  const rad = (d) => (d * Math.PI) / 180;
  const arc = new THREE.TorusGeometry(0.7, 0.08, 8, 48, rad(330));
  arc.rotateZ(rad(285));
  arc.rotateX(-HALF);
  geoShared = {
    plug: new THREE.CylinderGeometry(0.62, 0.62, 0.16, 24),
    hex: new THREE.CylinderGeometry(0.7, 0.7, 0.05, 6),
    arc,
    hashBox: new THREE.BoxGeometry(2.4, 1.0, 1.2),
  };
  matShared = { plug: materials.gold(), ring: materials.gold(), socket: materials.ink() };
}

// A pooled proof slab (edge, body, pins) that stands in for a kit slab at small scale.
function slabProp(init, n, pinPools, opts = {}) {
  const p = rig.prop(init);
  p.box('gold', 0, 0.05, 0, 3.14, 0.1, 2.14);
  p.box('titanium', 0, 0.25, 0, 3, 0.3, 2);
  for (let i = 0; i < n; i++) {
    const x = n > 1 ? -1.3 + (i * 2.6) / (n - 1) : 0;
    p.box(pinPools[i] || 'titanium', x, 0.25, 1.15, 0.22, 0.22, 0.3);
  }
  if (opts.ring) {
    const m = new THREE.Mesh(geoShared.arc, matShared.ring);
    m.position.y = 0.48;
    p.node.add(m);
  }
  if (opts.disc) {
    const m = new THREE.Mesh(geoShared.plug, matShared.plug);
    m.position.y = 0.5;
    p.node.add(m);
  }
  return p;
}

// Linking helpers.
const E = (p, x = 0, y = 0, z = 0) => ({ p, o: [x, y, z] });
function link(a, b, tr, thick) { return rig.link(a, b, tr, thick); }
function growLink(a, b, b0, s0, s1, thick, lv) {
  const g = scalar(0);
  g.go(T(b0, s0), T(b0, s1), [[1]], true);
  if (lv) g.go(T(lv[0], lv[1]), T(lv[0], lv[1] + 0.4), [[0]], true);
  return rig.link(a, b, g, thick);
}

function anchor(name, obj) { ctx.labels.anchor(name, obj); }
function statA(name, x, y, z) {
  const o = new THREE.Object3D();
  o.position.set(x, y, z);
  ctx.scene.add(o);
  anchor(name, o);
  return o;
}

// The hash motion (3.0 s) of storyboard 0.5 at hashD. items: [{ p, src, fs }]. out: { p, size }.
const FILE_Y = 1.0, FILE_Z = -2.2, SLOT_IN = [-0.2, 0.8, FILE_Z], SLOT_OUT = [2.2, 0.8, FILE_Z];
const fileX = (i) => -1.0 - 0.62 * i;
function hashRun(b, s0, items, out) {
  const n = items.length;
  items.forEach((it, i) => {
    const tp = T(b, s0);
    if (it.src) it.p.tr.jump(tp, it.src);
    else if (!it.keep) it.p.tr.jump(tp, [fileX(i), FILE_Y, FILE_Z, 0.001]);
    it.p.tr.go(tp, tp + 0.6, [[fileX(i), FILE_Y, FILE_Z, it.fs]]);
    const t1 = tp + 0.6 + (i * 0.9) / (n - 1);
    it.p.tr.go(t1, t1 + 0.3, [[SLOT_IN[0], SLOT_IN[1], SLOT_IN[2], 0.001]]);
  });
  glow.go(T(b, s0 + 1.8), T(b, s0 + 2.0), [[0.8]]);
  glow.go(T(b, s0 + 2.0), T(b, s0 + 2.2), [[0]]);
  out.p.tr.jump(T(b, s0 + 2.2), [SLOT_OUT[0], SLOT_OUT[1], SLOT_OUT[2], 0.2 * out.size]);
  out.p.tr.go(T(b, s0 + 2.2), T(b, s0 + 3.0), [[3.4, 1.0, FILE_Z, out.size]]);
}

export default {
  id: 'decider',
  title: 'The decider',
  beats: BEATS,

  build(c) {
    ctx = c;
    sharedGeos();
    rig = new Rig(ctx.scene);
    glow = scalar(0);
    rootVerify = scalar(0);
    rootDim = scalar(0);
    plugY = scalar(2);
    frames = [];
    segs = [];
    extraDisposables = [];
    const s = ctx.camera.shots;

    // ---- fixed set: housing, caliper posts, key rack ------------------------------------------------
    const st = rig.prop([0, 0, 0, 1]);
    st.box('ceramic', -2.5, 0.15, 0, 15, 0.3, 8);
    st.box('titanium', -2.5, 0.33, 3.94, 15, 0.06, 0.12);
    st.box('titanium', -2.5, 0.33, -3.94, 15, 0.06, 0.12);
    st.box('titanium', -9.94, 0.33, 0, 0.12, 0.06, 8);
    st.box('titanium', 4.94, 0.33, 0, 0.12, 0.06, 8);
    st.box('titanium', -9.65, 0.8, 0, 0.2, 1.0, 2.4);
    st.box('titanium', -6.35, 0.8, 0, 0.2, 1.0, 2.4);
    st.box('titanium', -6, 4.2, -5.4, 7, 0.15, 0.15);
    st.box('titanium', -9.4, 2.1, -5.4, 0.15, 4.2, 0.15);
    st.box('titanium', -2.6, 2.1, -5.4, 0.15, 4.2, 0.15);
    [[-8.2, 'gold', 3], [-6, 'goldLit', 2], [-3.8, 'goldLit', 1]].forEach(([x, pool, notches]) => {
      st.box(pool, x, 3.5, -5.4, 1.6, 1.0, 0.08);
      for (let i = 0; i < notches; i++) st.box('ink', x + (i - (notches - 1) / 2) * 0.4, 3.12, -5.34, 0.2, 0.2, 0.04);
    });
    statA('base', -2.5, BASE, -3.4);
    statA('cardNode', -6, 4.05, -5.4);
    statA('sealBase', -8, BASE, -1.0);

    // ---- root slab (arrives on the in plate) ---------------------------------------------------------
    rootSlab = makeProofSlab({ variant: 'node', pinKinds: ['collar', 'capcollar', 'capcollar', 'bar', 'bar'] });
    rootP = rig.prop([-20, 0.2, 0, 1]);
    rootP.node.add(rootSlab);
    rootP.tr.go(T(0, 2.0), T(0, 4.0), [[-8, BASE, 0]]);
    rootMats = [];
    const seen = new Set();
    rootSlab.traverse((o) => {
      if (o.material && !seen.has(o.material)) { seen.add(o.material); rootMats.push({ m: o.material, c: o.material.color.clone() }); }
    });
    rootSlab.userData.ring.updateMatrixWorld();
    anchor('root', rootP.anchor(-2.1, 0.5, 0));
    anchor('rootRing', rootP.anchor(0, 0.6, 0));
    anchor('ringJoint', rootP.anchor(0, 0.5, 0.75));
    anchor('ringTop', rootP.anchor(0, 0.95, 0));
    rootPlug = new THREE.Mesh(geoShared.plug, materials.gold());
    rootP.node.add(rootPlug);
    extraDisposables.push(() => rootPlug.material.dispose());
    plugY.go(T(11, 2.9), T(11, 3.0), [[1.8]]);
    plugY.go(T(11, 3.0), T(11, 3.6), [[0.48]]);
    // node card to root connector
    const cardLink = growLink(E(null, -6, 3.0, -5.4), E(null, -8, 0.88, -0.1), 0, 4.6, 5.0, 0.05);
    statA('cardLinkMid', -7, 1.95, -2.7);

    // ---- seal (4 pins, closed disc) ------------------------------------------------------------------
    sealSlab = makeProofSlab({ variant: 'decider', pinKinds: ['collar', 'collar', 'collar', 'bar'] });
    sealP = rig.prop([-8, 2.4, 0, 0]);
    sealP.node.add(sealSlab);
    const sealPlug = new THREE.Mesh(geoShared.plug, matShared.plug);
    sealPlug.position.y = 0.5;
    sealP.node.add(sealPlug);
    sealP.tr.go(T(11, 5.0), T(11, 6.4), [[-8, 0.95, 0, 1]]);
    sealP.tr.go(T(12, 5.0), T(12, 6.6), [[-6.5, 2.0, 0], [0.5, 2.0, 0], [2.0, 0.95, 0], [6.3, 0.95, 0], [8.5, 0.2, 0]]);
    sealP.tr.go(T(13, 8.0), T(13, 10.2), [[20, 0.2, 0]]);
    anchor('seal', sealP.anchor(0, 0.6, 0));

    // ---- beat 1: genesis requirement -----------------------------------------------------------------
    const gauge = rig.prop([-7.25, 0.35, 2.6, 0]);
    gauge.box('titanium', 0, 0, 0, 4.5, 0.1, 0.3);
    gauge.box('ink', -2.05, 0.055, 0, 0.4, 0.02, 0.34);
    [-1.75, -1.25, -0.75, -0.25, 0.75, 1.25].forEach((x) => gauge.box('line', x, 0.06, 0, 0.03, 0.02, 0.18));
    gauge.box('line', 0.35, 0.06, 0, 0.03, 0.02, 0.22, [0, 0.35, 0]);
    gauge.box('line', 0.5, 0.06, 0, 0.03, 0.02, 0.22, [0, 0.35, 0]);
    gauge.box('line', 2.2, 0.06, 0, 0.05, 0.02, 0.5);
    appear(gauge, 1, 1.0);
    const bar = rig.prop([-7.35, 0.55, 1.15, 0]);
    bar.box('titanium', 0, 0, 0, 0.34, 0.06, 0.3);
    bar.tr.jump(T(1, 2.0), [-7.35, 0.55, 1.15, 1]);
    bar.tr.go(T(1, 2.0), T(1, 2.8), [[-6.2, 0.9, 2.6]]);
    bar.tr.go(T(1, 3.0), T(1, 4.2), [[-9.3, 0.46, 2.6]]);
    const barFrame = rig.prop([-9.3, 0.5, 2.6, 0]).frame(0.46, 0.42, 0.04);
    barFrame.tr.jump(T(1, 4.0), [undefined, undefined, undefined, 1]);
    frames.push({ p: barFrame, t0: T(1, 4.0) });
    anchor('barNotch', barFrame.anchor(0, 0, -0.5));
    const leaf = slabProp([-10.6, 0.3, 2.6, 0], 7, [], {});
    appear(leaf, 1, 4.8, 0.4, 0.35);
    anchor('genLeaf', leaf.anchor(0, 0, -3.6));
    anchor('genLeafB', leaf.anchor(0, 0, -5.4));
    growLink(E(null, -9.3, 0.42, 2.6), E(null, -9.0, 0.4, 1.05), 1, 5.0, 5.7, 0.05);
    growLink(E(null, -9.0, 0.4, 1.05), E(null, -10.1, 0.4, 2.4), 1, 5.7, 6.4, 0.05);

    // ---- beat 2: the header opens ---------------------------------------------------------------------
    postTok = makeStateToken({ content: 'state:final', size: 1 });
    shareToken(postTok, 'state', 'state:final');
    postTok.rotation.x = -HALF;
    postP = rig.prop([-8, 0.7, 1.15, 0.001]);
    postP.node.add(postTok);
    postP.tr.go(T(2, 2.0), T(2, 3.0), [[-6, 2.0, -0.5, 1.2], [-3.5, 0.5, -2.2, 2.0]]);
    anchor('post', postP.anchor(0, 0, -0.6));
    anchor('postB', postP.anchor(0, 0, 0.6));
    postTok.userData.setState({ open: 0, lit: 0, highlight: -1 });
    const FX = [-5.5, -4.5, -3.5, -2.5, -1.5], FZ = -3.9;
    const f = [
      cellP('outcome', FX[0], 0.5, FZ, 0, 1),
      cellP('outcome', FX[1], 0.5, FZ, 0, 2),
      tokP('input', 'input:main', 0.8, FX[2], BASE + 0.07, FZ),
      tokP('state', 'deps:empty', 0.8, FX[3], BASE + 0.07, FZ),
      tokP('state', 'body:final', 0.8, FX[4], BASE + 0.07, FZ),
    ];
    const SEG_OF = [4, 3, 0, 2, 1];
    const rimPts = [4, 3, 0, 2, 1].map((_, k) => {
      const i = SEG_OF[k];
      const th = ((90 - i * 72) * Math.PI) / 180;
      return [-3.5 + 1.12 * Math.cos(th), BASE + 0.2, -2.2 - 1.12 * Math.sin(th)];
    });
    f.forEach((p, k) => {
      const arr = T(2, 3.6 + 0.3 * k + 0.6);
      p.tr.go(arr, arr + 0.3, [[undefined, undefined, undefined, p.size || 1]]);
      growLink(E(p, 0, 0, 0), E(null, rimPts[k][0], rimPts[k][1], rimPts[k][2]), 2, 4.5 + 0.3 * k, 4.9 + 0.3 * k, 0.04);
      anchor('fl' + k, p.anchor(0, 0, -0.5 - 0.6 * (k % 2)));
      segs.push({ i: SEG_OF[k], k, t0: T(2, 3.6 + 0.3 * k), tx: (FX[k] + 3.5) / 2, ty: -(FZ + 2.2) / 2, tz: (0.5 - 0.48) / 2 });
    });
    anchor('fieldsTray', f[2].anchor(0, 0, -1.4));
    const postFrame = rig.prop([-3.5, 0.62, -2.2, 0]).frame(2.5, 2.5, 0.06);
    postFrame.tr.jump(T(3, 6.4), [undefined, undefined, undefined, 1]);
    frames.push({ p: postFrame, t0: T(3, 6.4) });

    // ---- beat 3: boundary commitment -------------------------------------------------------------------
    hashP = rig.prop([1.0, 0.8, -2.2, 0]);
    hashMat = materials.gold();
    const hashMesh = new THREE.Mesh(geoShared.hashBox, hashMat);
    hashP.node.add(hashMesh);
    hashP.box('ink', -1.2, 0, 0, 0.12, 0.5, 0.6);
    hashP.box('ink', 1.2, 0, 0, 0.12, 0.5, 0.6);
    appear(hashP, 3, 0.5);
    extraDisposables.push(() => hashMat.dispose());
    anchor('hashAsm', hashP.anchor(0, 0.7, -0.7));
    statA('domMark', fileX(0), FILE_Y, FILE_Z - 1.5);
    statA('verMark', fileX(1), FILE_Y, FILE_Z - 0.4);
    const dom = cellP('gold', 0, 0, 0, 0, 0);
    const c0 = cellP('outcome', 0, 0, 0, 0, 1);
    const c1 = cellP('outcome', 0, 0, 0, 0, 2);
    const c2 = tokP('input', 'input:main', 0.55, 0, 0, 0);
    const c3 = tokP('state', 'deps:empty', 0.55, 0, 0, 0);
    const c4 = tokP('state', 'body:final', 0.55, 0, 0, 0);
    const o1 = tokP('state', 'state:final', 1.2, 0, 0, 0);
    hashRun(3, 2.0, [
      { p: dom, fs: 0.8 },
      { p: c0, src: [FX[0], 0.5, FZ, 0.001], fs: 0.8 },
      { p: c1, src: [FX[1], 0.5, FZ, 0.001], fs: 0.8 },
      { p: c2, src: [FX[2], BASE + 0.07, FZ, 0.001], fs: 0.55 },
      { p: c3, src: [FX[3], BASE + 0.07, FZ, 0.001], fs: 0.55 },
      { p: c4, src: [FX[4], BASE + 0.07, FZ, 0.001], fs: 0.55 },
    ], { p: o1, size: 1.2 });
    o1.tr.go(T(3, 5.4), T(3, 6.4), [[1.0, 1.9, -3.0], [-3.5, 0.7, -2.2]]);
    o1.tr.jump(T(3, 6.4), [undefined, undefined, undefined, 0.001]);
    anchor('hOut1', o1.anchor(0, 0, -0.9));
    // five connectors from the fields to post stay (from beat 2)

    // ---- beat 4: running state refused -----------------------------------------------------------------
    const gate = rig.prop([-1, 0.9, 1.6, 0]);
    gate.box('titanium', 0, 0, -0.7, 0.2, 1.2, 0.2);
    gate.box('titanium', 0, 0, 0.7, 0.2, 1.2, 0.2);
    gate.box('titanium', 0, 0.66, 0, 0.24, 0.12, 1.8);
    appear(gate, 4, 0.5);
    anchor('gate', gate.anchor(0, 0.8, -0.5));
    const ck = slabProp([-3.2, 0.3, 2.8, 0], 5, [], { ring: true });
    appear(ck, 4, 1.0, 0.4, 0.5);
    anchor('ckpt', ck.anchor(0, 0, -1.2));
    const run = rig.prop([-3.0, 0.5, 1.6, 0]);
    run.box('step', 0, 0, 0, 0.4, 0.4, 0.4);
    [-0.12, 0, 0.12].forEach((o) => run.box('error', o, 0.21, 0, 0.06, 0.02, 0.5, [0, Math.PI / 4, 0], 1));
    appear(run, 4, 2.0, 0.3);
    run.tr.go(T(4, 2.4), T(4, 2.7), [[-1.8]]);
    run.tr.go(T(4, 2.7), T(4, 3.0), [[-2.2]]);
    run.hatchAt = T(4, 3.0);
    anchor('runCell', run.anchor(0, 0, -0.5));
    const wait = cellP('outcome', -3.7, 0.5, 1.6, 0, 1);
    appear(wait, 4, 2.0, 0.3);
    leave(wait, 5, 0.0);
    leave(run, 5, 0.0);
    leave(ck, 5, 0.0);

    // ---- beat 5: every outcome seals ---------------------------------------------------------------------
    const inset = rig.prop([10.5, -0.05, -1.5, 0.001]);
    inset.box('paper', 0, 0, 0, 9, 0.1, 4.5);
    inset.box('titanium', 0, 0.05, 2.2, 9, 0.1, 0.12);
    inset.box('titanium', 0, 0.05, -2.2, 9, 0.1, 0.12);
    inset.box('titanium', 4.44, 0.05, 0, 0.12, 0.1, 4.5);
    inset.box('titanium', -4.44, 0.05, 0, 0.12, 0.1, 4.5);
    const rise = (b) => inset.tr.go(T(b, 0), T(b, 0.6), [[10.5, 0.05, -1.5, 1]]);
    const sink = (b) => inset.tr.go(T(b, 0), T(b, 0.6), [[10.5, -0.05, -1.5, 0.001]]);
    rise(5); sink(6); rise(7); sink(8);
    const rowZ = [-2.8, -1.5, -0.2];
    const rows = [];
    const outs = ['outcome:main', 'outcome:rejected', 'outcome:exhausted'];
    rowZ.forEach((z, r) => {
      const term = rig.prop([7.5, 0.15, z, 0]);
      term.box(r ? 'error' : 'outcome', 0, 0, 0, 0.7, 0.1, 0.5);
      const pay = cellP('ceramic', 8.5, 0.3, z, 0, r + 1);
      const ms = slabProp([11, 0.1, z, 0], 4, ['program', 'input', 'outcome', 'titanium'], { disc: true });
      const tk = tokP('outcome', outs[r], 0.8, 13.5, 0.17, z);
      const fr = rig.prop([7.5, 0.22, z, 0]).frame(0.85, 0.65, 0.04);
      rows.push({ term, pay, ms, tk });
      appear(term, 5, 0.9); appear(pay, 5, 1.0); appear(ms, 5, 1.1, 0.4, 0.4); appear(tk, 5, 1.2, 0.4, 0.8);
      const tf = T(5, 2.0 + (0.5 + z / 6.6 * 0.5) * 1.0 + 0.2);
      fr.tr.jump(tf, [undefined, undefined, undefined, 1]);
      frames.push({ p: fr, t0: tf });
      [term, pay, ms, tk, fr].forEach((p) => leave(p, 6, 0.0));
      growLink(E(term, 0.35, 0, 0), E(pay, -0.2, 0, 0), 5, 1.4, 1.8, 0.04, [6, 0]);
      growLink(E(pay, 0.2, 0, 0), E(ms, -1.2 * 0.4, 0, 0), 5, 1.5, 1.9, 0.04, [6, 0]);
      growLink(E(ms, 1.2 * 0.4, 0, 0), E(tk, -0.4, 0, 0), 5, 1.6, 2.0, 0.04, [6, 0]);
      anchor('row' + (r + 1), term.anchor(0, 0, -0.5));
      anchor('tok' + (r + 1), tk.anchor(0, 0, -0.7));
    });
    const swp = rig.prop([7.5, 0.25, -3.3, 0.001]);
    swp.box('sweep', 0, 0, 0, 1.0, 0.03, 0.08);
    swp.tr.jump(T(5, 2.0), [undefined, undefined, undefined, 1]);
    swp.tr.go(T(5, 2.0), T(5, 3.2), [[undefined, undefined, 0.1]], true);
    swp.tr.jump(T(5, 3.3), [undefined, undefined, undefined, 0.001]);

    // ---- beat 6: outcome digest ----------------------------------------------------------------------------
    const ver = cellP('ceramic', 0, 0, 0, 0, 0);
    const o2 = tokP('outcome', 'outcome:main', 1.2, 0, 0, 0);
    const tProg = tokP('program', 'prog:main', 0.8, 1.4, 0.6, -0.6);
    const tIn = tokP('input', 'input:main', 0.8, 2.4, 0.6, -0.6);
    appear(tProg, 6, 6.0, 0.4, 0.8); appear(tIn, 6, 6.0, 0.4, 0.8);
    const tag = cellP('outcome', 0, 0, 0, 0, 1), pay = cellP('outcome', 0, 0, 0, 0, 2);
    const plate = rig.prop([1.0, 0.33, -3.6, 0]);
    plate.box('ceramic', 0, 0, 0, 1.8, 0.06, 0.6);
    plate.box('ink', 0, 0.035, 0, 1.2, 0.02, 0.08);
    appear(plate, 6, 0.6);
    leave(plate, 7, 0.0);
    anchor('encPlate', plate.anchor(0, 0, -0.5));
    // tag copy passes through the gate to the file
    tag.tr.jump(T(6, 1.6), [FX[0], 0.5, FZ, 0.8]);
    tag.tr.go(T(6, 1.6), T(6, 2.4), [[-2.8, 0.5, 1.6], [-0.2, 0.5, 1.6], [fileX(2), FILE_Y, FILE_Z, 0.8]]);
    hashRunCustom(6, 2.6, [ver, null, tag, pay], o2);
    function hashRunCustom(b, s0, order, out) {
      // order: dom, ver, tag, payload; tag is already near the file (its own track), dom is the shared gold cell.
      const items = [{ p: dom, fs: 0.8 }, { p: order[0], fs: 0.8 }, { p: order[2], fs: 0.8, keep: true }, { p: order[3], src: [FX[1], 0.5, FZ, 0.001], fs: 0.8 }];
      hashRun(b, s0, items, { p: out, size: 1.2 });
    }
    o2.tr.go(T(6, 6.0), T(6, 7.0), [[3.4, 0.6, -0.6]]);
    anchor('hOut2', o2.anchor(0, 0, -0.9));
    anchor('fp2', tProg.anchor(0, 0, -0.7));
    anchor('fp3', tIn.anchor(0, 0, -0.7));
    anchor('fp4', o2.anchor(0, 0, -0.9));

    // ---- beat 7: equal values, equal faces -----------------------------------------------------------------
    const tileL = [[6.6, 0.3], [9.4, 0.3], [8.0, -0.7]].map(([x, z], i) => {
      const t = rig.prop([x, 0.17, z, 0]);
      t.box('ceramic', 0, 0, 0, 0.8, 0.12, 0.8);
      t.box('ink', 0, 0.07, 0, 0.5, 0.02, 0.1);
      appear(t, 7, 1.0 + 0.1 * i);
      leave(t, 8, 0.0);
      return t;
    });
    const tileR = [11.2, 12.4, 13.6, 14.8].map((x, i) => {
      const t = rig.prop([x, 0.17, -0.5, 0]);
      t.box('ceramic', 0, 0, 0, 0.8, 0.12, 0.8);
      t.box('ink', 0, 0.07, 0, 0.5, 0.02, 0.1);
      appear(t, 7, 1.0 + 0.1 * i);
      leave(t, 8, 0.0);
      growLink(E(t, 0, 0, -0.4), E(t, 0, 0, -0.8), 7, 1.3, 1.7, 0.04, [8, 0]);
      return t;
    });
    const msL = slabProp([8, 0.1, -2.8, 0], 4, ['program', 'input', 'outcome', 'titanium'], { disc: true });
    const msR = slabProp([13, 0.1, -2.8, 0], 4, ['program', 'input', 'outcome', 'titanium'], { disc: true });
    const tkL = tokP('outcome', 'outcome:same', 0.8, 8, 0.17, -1.6);
    const tkR = tokP('outcome', 'outcome:same', 0.8, 13, 0.17, -1.6);
    [msL, msR].forEach((m) => { appear(m, 7, 0.9, 0.4, 0.4); leave(m, 8, 0.0); });
    [tkL, tkR].forEach((m) => { appear(m, 7, 1.0, 0.4, 0.8); });
    leave(tkL, 8, 0.0);
    growLink(E(tileL[0], 0.4, 0, 0), E(tileL[2], -0.4, 0, 0), 7, 1.4, 1.8, 0.04, [8, 0]);
    growLink(E(tileL[1], -0.4, 0, 0), E(tileL[2], 0.4, 0, 0), 7, 1.4, 1.8, 0.04, [8, 0]);
    growLink(E(tileL[2], 0, 0, -0.4), E(tkL, 0, 0, 0.4), 7, 1.5, 1.9, 0.04, [8, 0]);
    growLink(E(tkL, 0, 0, -0.4), E(msL, 0, 0, 0.5), 7, 1.5, 1.9, 0.04, [8, 0]);
    growLink(E(null, 13, 0.17, -2.0), E(msR, 0, 0, 0.5), 7, 1.5, 1.9, 0.04, [8, 0]);
    tkR.tr.go(T(7, 2.0), T(7, 3.2), [[8, 0.17, -1.6]]);
    tkR.tr.jump(T(7, 3.2), [undefined, undefined, undefined, 0.001]);
    const eqFr = rig.prop([8, 0.3, -1.6, 0]).frame(1.0, 1.0, 0.05);
    eqFr.tr.jump(T(7, 3.2), [undefined, undefined, undefined, 1]);
    leave(eqFr, 8, 0.0);
    frames.push({ p: eqFr, t0: T(7, 3.2) });
    anchor('shareDiag', tileL[2].anchor(0, 0, -0.5));
    anchor('sepDiag', tileR[1].anchor(0, 0, -0.5));
    statA('insetMid', 10.5, 0.1, -3.75);
    anchor('tokSame1', tkL.anchor(0, 0, -0.7));

    // ---- beat 8: claim digest --------------------------------------------------------------------------------
    const cpProg = tokP('program', 'prog:main', 0.55, 0, 0, 0);
    const cpOut = tokP('outcome', 'outcome:main', 0.55, 0, 0, 0);
    const ver2 = cellP('ceramic', 0, 0, 0, 0, 1);
    const o3 = tokP('state', 'claim:main', 1.2, 0, 0, 0);
    const c2b = tokP('input', 'input:main', 0.55, 0, 0, 0);
    hashRun(8, 2.0, [
      { p: dom, fs: 0.8 },
      { p: ver2, fs: 0.8 },
      { p: cpProg, src: [1.4, 0.6, -0.6, 0.001], fs: 0.55 },
      { p: c2b, src: [2.4, 0.6, -0.6, 0.001], fs: 0.55 },
      { p: cpOut, src: [3.4, 0.6, -0.6, 0.001], fs: 0.55 },
    ], { p: o3, size: 1.2 });
    o3.tr.go(T(8, 5.4), T(8, 6.4), [[3.4, 0.5, 0.8]]);
    const sock = new THREE.Mesh(geoShared.hex, matShared.socket);
    const sockP = rig.prop([3.4, 0.32, 0.8, 0]);
    sockP.node.add(sock);
    [0, 1, 2, 3, 4, 5].forEach((k) => {
      const a = (k * Math.PI) / 3;
      sockP.box('titanium', 0.7 * Math.cos(a) * 0.87, 0.03, 0.7 * Math.sin(a) * 0.87, 0.06, 0.08, 0.7, [0, -a + Math.PI / 2, 0]);
    });
    appear(sockP, 8, 0.6);
    anchor('hOut3', o3.anchor(0, 0, -0.9));

    // ---- beat 9: dependencies ----------------------------------------------------------------------------------
    const dFrame = rig.prop([FX[3], BASE + 0.2, FZ, 0]).frame(1.0, 1.0, 0.05);
    dFrame.sel = 1;
    appear(dFrame, 9, 0.3, 0.3);
    leave(dFrame, 10, 0.0);
    const dCopy = tokP('state', 'deps:list', 0.8, -0.6, BASE + 0.07, FZ);
    const lTok = tokP('state', 'deps:list', 0.8, 0.5, BASE + 0.07, FZ);
    const dSlab = slabProp([1.6, BASE, -4.4, 0], 5, [], {});
    const claims = [0.9, 1.6, 2.3].map((x) => tokP('state', 'claim:dep', 0.5, x, BASE + 0.05, -5.4));
    [dCopy, lTok, ...claims].forEach((p) => appear(p, 9, 1.0, 0.4, p.size));
    appear(dSlab, 9, 1.0, 0.4, 0.4);
    [dCopy, lTok, ...claims].forEach((p) => leave(p, 10, 0.0));
    leave(dSlab, 10, 0.0);
    claims.forEach((cl) => growLink(E(dSlab, 0, 0, -0.45), E(cl, 0, 0, 0.25), 9, 1.4, 1.9, 0.04, [10, 0]));
    growLink(E(lTok, 0.4, 0, 0), E(dSlab, -0.5, 0, 0), 9, 1.4, 1.9, 0.04, [10, 0]);
    lTok.tr.go(T(9, 2.0), T(9, 3.2), [[-0.6]]);
    lTok.tr.jump(T(9, 3.2), [undefined, undefined, undefined, 0.001]);
    const dEq = rig.prop([-0.6, BASE + 0.2, FZ, 0]).frame(1.0, 1.0, 0.05);
    dEq.tr.jump(T(9, 3.2), [undefined, undefined, undefined, 1]);
    leave(dEq, 10, 0.0);
    frames.push({ p: dEq, t0: T(9, 3.2) });
    const emptyTray = rig.prop([-2.5, BASE + 0.03, -4.9, 0]);
    emptyTray.box('titanium', 0, 0, 0.45, 0.9, 0.06, 0.05);
    emptyTray.box('titanium', 0, 0, -0.45, 0.9, 0.06, 0.05);
    emptyTray.box('titanium', 0.45, 0, 0, 0.05, 0.06, 0.9);
    emptyTray.box('titanium', -0.45, 0, 0, 0.05, 0.06, 0.9);
    appear(emptyTray, 9, 1.0);
    leave(emptyTray, 10, 0.0);
    statA('depsRoute', 0.6, BASE, -5.3);
    statA('depsSlab', 1.3, BASE, -3.7);
    anchor('claimToks', claims[1].anchor(0, 0, 1.4));
    anchor('depsMark', f[3].anchor(0, 0, -0.5));
    anchor('emptyTray', emptyTray.anchor(0, 0, -0.5));

    // ---- beat 10: step count --------------------------------------------------------------------------------------
    const CX = -4.5;
    const collar = rig.prop([CX, BASE + 0.06, 3.4, 0]);
    collar.box('titanium', 0, 0, 0, 1.9, 0.12, 0.3);
    for (let i = 0; i < 8; i++) collar.box('ink', (i - 3.5) * 0.22, 0.065, 0, 0.12, 0.01, 0.12);
    appear(collar, 10, 0.5);
    anchor('collar', collar.anchor(0, 0, -0.6));
    const strip = rig.prop([CX, 0.03, 4.2, 0]);
    strip.box('titanium', 0, 0, 0, 4.2, 0.06, 0.55);
    for (let i = 0; i < 8; i++) strip.box('ceramic', (i - 3.5) * 0.5, 0.05, 0, 0.4, 0.06, 0.4);
    appear(strip, 10, 3.0);
    anchor('strip', strip.anchor(2.6, 0, 0.1));
    for (let i = 0; i < 8; i++) {
      const cu = rig.prop([CX + (i - 3.5) * 0.22, BASE + 0.22, 3.4, 0]);
      cu.box('ceramic', 0, 0, 0, 0.2, 0.2, 0.2);
      appear(cu, 10, 0.5, 0.4, 1);
      const tx = CX + (i - 3.5) * 0.5;
      cu.tr.go(T(10, 3.4 + 0.05 * i), T(10, 4.0 + 0.05 * i), [[tx, 0.22, 4.2]]);
      growLink(E(null, tx, 0.22, 4.2), E(null, CX + (i - 3.5) * 0.22, BASE + 0.3, 3.4), 10, 4.0 + 0.04 * i, 4.5 + 0.04 * i, 0.03);
    }
    statA('recombMark', CX + 2.2, 0.3, 3.7);
    const hh = rig.prop([-6.7, 0.55, 1.15, 0.001]);
    hh.box('titanium', 0, 0, 0, 0.34, 0.06, 0.3);
    hh.tr.jump(T(10, 2.0), [-6.7, 0.55, 1.15, 1]);
    hh.tr.go(T(10, 2.0), T(10, 3.0), [[-7.5, 0.8, 2.6], [-5.8, 0.55, 3.4]]);
    growLink(E(hh, 0.17, 0, 0), E(collar, -0.95, 0.1, 0), 10, 3.0, 3.4, 0.04);
    anchor('hhiBar', hh.anchor(0, 0, -0.5));
    const sw2 = rig.prop([CX - 1.75, 0.12, 4.2, 0.001]);
    sw2.box('sweep', 0, 0, 0, 0.08, 0.02, 0.7);
    sw2.tr.jump(T(10, 5.0), [undefined, undefined, undefined, 1]);
    sw2.tr.go(T(10, 5.0), T(10, 5.7), [[CX + 1.75]], true);
    sw2.tr.jump(T(10, 5.8), [undefined, undefined, undefined, 0.001]);
    const ruler = rig.prop([-3.5, 0.05, 5.0, 0]);
    ruler.box('titanium', 0, 0, 0, 5, 0.1, 0.3);
    [-2.0, -1.5, -1.0, -0.5, 0.0, 0.5, 1.0].forEach((x) => ruler.box('line', x, 0.06, 0, 0.03, 0.02, 0.18));
    ruler.box('line', 1.45, 0.06, 0, 0.03, 0.02, 0.22, [0, 0.35, 0]);
    ruler.box('line', 1.6, 0.06, 0, 0.03, 0.02, 0.22, [0, 0.35, 0]);
    ruler.box('line', 2.5, 0.06, 0, 0.06, 0.02, 0.5);
    appear(ruler, 10, 5.8);
    anchor('rulerEnd', ruler.anchor(2.5, 0, -0.4));

    // ---- beat 11: closed accumulator, beat 12 wall and public values ----------------------------------------------------
    const wall = rig.prop([5, 8, 0, 0.001]);
    wall.tr.go(T(12, 3.4), T(12, 3.8), [[5, 8, 0, 1]]);
    wall.tr.go(T(12, 3.8), T(12, 4.6), [[5, 1.6, 0]]);
    const WB = [
      [0, 0.85, 0, 0.2, 1.5, 8],
      [0, 0, -2.65, 0.2, 0.2, 2.7], [0, 0, 2.65, 0.2, 0.2, 2.7],
      [0, -1.4, -2.65, 0.2, 0.4, 2.7], [0, -1.4, 2.65, 0.2, 0.4, 2.7],
    ];
    WB.forEach((w) => wall.box('titanium', ...w));
    [[-2.8, 0.1], [-1.65, 0.35], [1.325, 0.05], [2.5, 0.1], [3.825, 0.35]].forEach(([z, w]) => wall.box('titanium', 0, -0.65, z, 0.2, 1.1, w));
    const AP = [-3.4, -2.2, 1.9, 3.1];
    [0, 1, 2].forEach((k) => {
      for (let q = 0; q < 6; q++) {
        const psi = (q * 60 * Math.PI) / 180;
        wall.box('ink', -0.11, -0.65 + 0.45 * Math.sin(psi), AP[k] + 0.45 * Math.cos(psi), 0.04, 0.06, 0.52, [-psi - HALF, 0, 0]);
      }
    });
    wall.box('ink', -0.11, -0.15, AP[3], 0.04, 0.05, 1.0);
    wall.box('ink', -0.11, -1.15, AP[3], 0.04, 0.05, 1.0);
    wall.box('ink', -0.11, -0.65, AP[3] - 0.5, 0.04, 1.0, 0.05);
    wall.box('ink', -0.11, -0.65, AP[3] + 0.5, 0.04, 1.0, 0.05);
    for (let i = 0; i < 9; i++) wall.box('ink', -0.12, 0.85, -3.6 + 0.9 * i, 0.02, 0.04, 1.3, [Math.PI / 4, 0, 0]);
    anchor('wallMark', wall.anchor(-0.1, 1.2, 1.0));

    const privBar = rig.prop([2.6, 0.3, 0, 0]);
    privBar.box('titanium', 0, 0, 0, 0.3, 0.1, 7.2);
    appear(privBar, 12, 1.6, 0.4);
    leave(privBar, 12, 5.0);
    const pubBar = rig.prop([8.5, 0.3, 2.8, 0]);
    pubBar.box('titanium', 0, 0, 0, 4.6, 0.1, 0.6);
    appear(pubBar, 12, 5.0, 0.4);
    leave(pubBar, 13, 2.0, 0.6);
    const nPub = rig.prop([CX, 0.5, 3.4, 0]);
    nPub.box('titanium', 0, 0, 0, 1.9, 0.12, 0.3);
    for (let i = 0; i < 8; i++) nPub.box('ceramic', (i - 3.5) * 0.22, 0.16, 0, 0.2, 0.2, 0.2);
    nPub.tr.jump(T(12, 2.0), [CX, 0.5, 3.4, 0.001]);
    const PUBX = [7.0, 8.0, 9.0, 10.2];
    const items = [tProg, tIn, o2, nPub];
    const ZB = [AP[0], AP[1], AP[2], AP[3]];
    items.forEach((p, k) => {
      const sz = p === nPub ? 0.7 : 0.8;
      p.tr.go(T(12, 2.0), T(12, 3.0), [[2.6, 0.5, ZB[k], sz]]);
      p.tr.go(T(12, 5.0), T(12, 6.6), [[4.0, 0.7, ZB[k]], [5.0, 0.95, ZB[k]], [5.9, 0.7, ZB[k]], [PUBX[k], 0.5, 2.8]]);
      p.tr.go(T(13, 2.0), T(13, 2.6), [[undefined, 0.1, undefined, 0]]);
    });
    anchor('pubProg', tProg.anchor(0, 0, -0.7));
    anchor('pubIn', tIn.anchor(0, 0, -0.7));
    anchor('pubOut', o2.anchor(0, 0, -0.7));
    anchor('pubSteps', nPub.anchor(0, 0, -0.6));

    // ---- beat 13: budget ----------------------------------------------------------------------------------------------
    const budget = rig.prop([10.5, 0.06, 5.6, 0]);
    budget.box('titanium', 0, 0, 0.15, 8, 0.12, 0.04);
    budget.box('titanium', 0, 0, -0.15, 8, 0.12, 0.04);
    budget.box('titanium', 3.98, 0, 0, 0.04, 0.12, 0.3);
    budget.box('titanium', -3.98, 0, 0, 0.04, 0.12, 0.3);
    budget.box('line', 0, 0.02, -0.45, 8, 0.04, 0.05);
    budget.box('line', 4, 0.02, -0.3, 0.05, 0.04, 0.3);
    budget.box('line', -4, 0.02, -0.3, 0.05, 0.04, 0.3);
    appear(budget, 13, 0.5);
    anchor('budget', budget.anchor(0, 0, -0.6));

    // camera
    const pose = {
      IN: s.establish([-10, 1, 0], 30), GAUGE: s.inspect([-7.3, 0, 1.4], 9, 'top'), HEAD: s.inspect([-3, 0, -3.0], 10, 'top'),
      HASHV: s.inspect([-0.5, 0, -2.4], 13, 'top'), GATE: s.establish([-1, 0.8, 1.6], 8), INSET: s.establish([10.5, 0.6, -1.5], 12),
      COUNT: s.inspect([-4, 0, 3.6], 9, 'top'), RING: s.establish([-8, 0.6, 0], 7), WALL: s.establish([4.5, 1, 0.5], 22),
      OUT: s.establish([10, 1, 0], 30),
    };
    const seq = ['IN', 'GAUGE', 'HEAD', 'HASHV', 'GATE', 'INSET', 'HASHV', 'INSET', 'HASHV', 'HEAD', 'COUNT', 'RING', 'WALL', 'WALL'];
    const keys = seq.map((n, b) => ({ t: ST[b], pose: pose[n], dur: b === 0 ? 2 : 1.5 })).filter((k, b) => b === 0 || seq[b] !== seq[b - 1]);
    keys.push({ t: T(13, 6.0), pose: pose.OUT, dur: 1.5 });
    ctx.camera.track(keys);
    keys.length = 0;

    runP = run;
    rig.finalize();
  },

  seek(t) {
    const b = ctx.beatAt(t);
    rig.update(t);

    // frame flashes: lit for 0.3 s after the frame appears, then plain gold
    for (const fr of frames) fr.p.sel = t >= fr.t0 && t < fr.t0 + 0.3 ? 1 : 0;
    // hatch on the Running cell
    runP.sel = t >= runP.hatchAt ? 1 : 0;

    // hash block glow
    hashMat.emissive.setHex(PALETTE.goldBright);
    hashMat.emissiveIntensity = val(glow, t);

    // root slab
    const v = ramp(t, T(0, 5.0), T(0, 5.7));
    const edge = t >= T(0, 5.7) && t < T(0, 6.0) ? 1 : 0;
    const closed = phase(t, T(11, 2.0), T(11, 3.0));
    rootSlab.userData.setState({ pins: ['program', 'state', 'state', null, null], verify: v, edgeLit: edge, ring: { show: 1, folds: 2, closed } });
    rootPlug.visible = t >= T(11, 2.9);
    rootPlug.position.y = val(plugY, t);
    rootPlug.material.emissive.setHex(PALETTE.goldBright);
    rootPlug.material.emissiveIntensity = t >= T(11, 3.6) && t < T(11, 3.9) ? 0.8 : 0;
    const dim = 0.6 * phase(t, T(11, 5.0), T(11, 6.4));
    for (const r of rootMats) { if (dim > 0) { r.m.color.copy(r.c).lerp(dimC, dim); } }

    // seal pins
    sealSlab.userData.setState({ pins: t >= T(13, 1.0) ? ['program', 'input', 'outcome', null] : [], ring: { show: 1, folds: 0, closed: 1 } });

    // post token: segments leave the rim one by one, return in beat 3
    postTok.userData.setState({ open: 0, lit: 0, highlight: -1 });
    const R = 0.56;
    for (const sg of segs) {
      const g = postTok.userData.segments[sg.i];
      const e = phase(t, sg.t0, sg.t0 + 0.6);
      const back = ramp(t, T(3, 6.2), T(3, 6.6));
      const th = ((90 - sg.i * 72) * Math.PI) / 180;
      const cx = R * Math.cos(th), cy = R * Math.sin(th);
      const useE = t >= T(3, 6.2) ? 0 : e;
      g.position.set(lerp(cx, sg.tx, useE), lerp(cy, sg.ty, useE), lerp(0.1, sg.tz + 0.1, useE));
      g.rotation.z = th;
      const sca = t >= T(3, 6.2) ? Math.max(back, 0.001) : lerp(1, 0.5, useE);
      g.scale.setScalar(sca);
      g.visible = useE < 0.999;
    }

    // carrier
    ctx.carrier.show('prog_d', 1, 'prog:main');
    ctx.carrier.show('input_d', 1, 'input:main');
    const k = phase(t, T(12, 7.0), T(12, 8.0));
    ctx.carrier.show('outcome_d', t >= T(13, 0) ? 1 : k, 'outcome:main');
    ctx.carrier.show('n_steps', t >= T(13, 0) ? 1 : k, 'n_steps:main');
    void b;
  },

  dispose() {
    for (const k of [rootSlab, sealSlab, postTok]) if (k && k.userData.dispose) k.userData.dispose();
    for (const d of extraDisposables || []) d();
    if (geoShared) { Object.values(geoShared).forEach((g) => g.dispose()); geoShared = null; }
    if (matShared) { Object.values(matShared).forEach((m) => m.dispose()); matShared = null; }
    if (rig) rig.dispose();
    disposeShared();
    ctx = null; rig = null; rootP = null; rootSlab = null; sealSlab = null; postTok = null; hashP = null; hashMat = null;
    frames = null; segs = null;
  },
};

const dimC = new THREE.Color(PALETTE.paperWarm);
