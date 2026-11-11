// Scene 4: the node. MERGE of two segments, the fold into one accumulator, linear work,
// parallel merge trees, MERGE-WORK, JOIN and DAG, the continuation checkpoint, the root handoff.
// Every pose is a pure function of the scene time t. Positions are written against the beat start S[i].
import * as THREE from 'three/webgpu';
import { PALETTE, FAMILY, materials, makeDigestToken, lerp, phase, ramp, clamp01 } from '../theme.js';
import { makeProofSlab } from '../kit.js';
import { COPY } from './scene4/copy.js';
import { Parts, disposeGroup } from './scene4/parts.js';

const S = [];
{ let a = 0; for (const c of COPY) { S.push(a); a += c.duration; } }

let ctx = null;
let P = null;
let O = null;
let tt = 0;
let bi = 0;

const ph = (i, a, b) => phase(tt, S[i] + a, S[i] + b);
const rp = (i, a, b) => ramp(tt, S[i] + a, S[i] + b);
const ap = (i, a) => ph(i, a, a + 0.4);              // appear
const lv = (i, a) => 1 - ph(i, a, a + 0.4);          // leave
const mix = (a, b, k) => a + (b - a) * k;
const rad = (d) => (d * Math.PI) / 180;

// ---- layout constants
const SL = [-8, 0.3, 0];
const SR = [-4, 0.3, 0];
const OUT = [1.4, 0.2, 0];
const RAIL_Z = 3.9;
const PROV = [[6.5, 4.7], [10.25, 4.7], [14, 4.7]];
const LEAF_T = [[6.5, 0.8], [9, 0.8], [11.5, 0.8], [14, 0.8]];
const NODE_T = [[7.75, -0.9], [12.75, -0.9]];
const ROOT_T = [10.25, -2.7];
const MINI = 0.35;
const TOK = 0.6;

const KINDS_LEAF = ['collar', 'collar', 'capcollar', 'capcollar', 'collar', 'bar', 'bar'];
const KINDS_NODE = ['collar', 'capcollar', 'capcollar', 'bar', 'bar'];
const FAM_LEAF = ['state', 'program', 'state', 'state', 'state', null, null];
const FAM_NODE = ['program', 'state', 'state', null, null];

// Label table: one row per storyboard label, same order as COPY[i].labels.
// [anchor name, kind, dy, from, to]
const LAB = [
  [['aLeaf', 'input', 1.0], ['aNodeR', 'state', 1.0], ['aRack', 'state', 0.7], ['aHouse', 'neutral', 0.2]],
  [['aActive', 'state', 0.2], ['aPins', 'state', 0.2], ['aKey', 'state', 0.2], ['aNeutral', 'neutral', 0.2]],
  [['aHPre', 'neutral', 0.2], ['aSteps', 'neutral', 0.2], ['aHLo', 'neutral', 0.2, 3.4], ['aHHi', 'neutral', 0.2, 3.6], ['aCounter', 'neutral', 0.2, 4.8], ['aRuler', 'neutral', 0.2, 4.8]],
  [['aHouse', 'step', 0.7], ['aLeft', 'state', 0.2], ['aRight', 'state', 0.2], ['aProg', 'program', 1.0, 3.4]],
  [['aPost', 'state', 0.9], ['aPre', 'state', 0.9], ['aEqual', 'state', 1.8, 3.2]],
  [['aLHi', 'neutral', 0.2], ['aRLo', 'neutral', 0.2], ['aEqH', 'neutral', 0.2, 3.2]],
  [['aPin0', 'program', 1.0, 5.0], ['aPin1', 'state', 1.7, 5.2], ['aPin2', 'state', 1.0, 5.4], ['aPin3', 'neutral', 1.7, 5.6], ['aPin4', 'neutral', 1.0, 5.6]],
  [['aConns', 'state', 0.5, 3.0], ['aFold', 'state', 1.0, 3.4], ['aRing', 'state', 0.7, 4.8], ['aJoint', 'state', 0.5, 4.8]],
  [['aFootprint', 'state', 0.2], ['aRingOutline', 'state', 0.2], ['aRSteps', 'step', 0.2, 4.4], ['aRLeaf', 'input', 0.2, 4.4], ['aRNode', 'state', 0.2, 4.4], ['aRLinear', 'state', 0.2, 6.0]],
  [['aProver', 'neutral', 1.8], ['aTime', 'neutral', 1.1, 2.8], ['aThreads', 'neutral', 0.7, 2.8], ['aMem', 'neutral', 0.3, 2.8]],
  [['aProvers', 'step', 1.8], ['aReady', 'state', 0.2], ['aOrder', 'neutral', 0.3, 4.4], ['aThru', 'neutral', 0.3, 5.0]],
  [['aHouse', 'step', 0.7], ['aPrior', 'state', 0.2], ['aNew', 'input', 0.2, 3.4], ['aProg', 'program', 0.8, 5.6], ['aPost', 'state', 0.8, 5.0], ['aOutH', 'neutral', 0.2, 5.8]],
  [['aJoin', 'step', 0.6], ['aDomain', 'state', 0.4], ['aL', 'state', 0.8], ['aR', 'state', 0.8], ['aSetRoot', 'state', 0.9, 5.4]],
  [['aClaim', 'state', 0.7], ['aCons', 'state', 0.2], ['aDag', 'neutral', 0.2], ['aShape', 'state', 0.7, 4.4], ['aKeys', 'state', 0.2]],
  [['aStore', 'step', 0.9], ['aCkpt', 'state', 0.6, 3.8], ['aPrefix', 'state', 0.2], ['aWit', 'neutral', 0.2], ['aPostP', 'state', 1.0], ['aKeySet', 'state', 0.2, 3.8]],
  [['aPrefix', 'state', 0.2], ['aSuffix', 'state', 0.2, 3.4], ['aPostP', 'state', 1.0], ['aPreS', 'state', 1.0, 3.4], ['aSavedH', 'neutral', 0.2, 4.6], ['aSame', 'state', 0.7, 4.6]],
  [['aRoot', 'state', 1.5], ['aOuter', 'state', 0.3], ['aRing', 'state', 0.3], ['aJoint', 'state', 0.3]],
];

const ANCHORS = [...new Set(LAB.flat().map((r) => r[0]))];

function buildBeats() {
  return COPY.map((c, i) => ({
    id: c.id,
    title: c.title,
    duration: c.duration,
    narration: c.narration,
    where: 'off',
    labels: c.labels.map((l, k) => {
      const r = LAB[i][k];
      const e = { id: `l${k}`, text: l.text, anchor: r[0], kind: l.kind || r[1], dy: r[2] };
      if (r[3] != null) e.from = r[3];
      if (r[4] != null) e.to = r[4];
      return e;
    }),
  }));
}

// ---- small pose helpers
function putTok(g, x, y, z, k = 1, flat = false) {
  const on = k > 0.001;
  g.visible = on;
  if (!on) return;
  g.position.set(x, y, z);
  g.scale.setScalar(k);
  g.rotation.x = flat ? -Math.PI / 2 : 0;
}
function putSlab(g, x, y, z, s = 1) {
  const on = s > 0.001;
  g.visible = on;
  if (!on) return;
  g.position.set(x, y, z);
  g.scale.setScalar(s);
}
function at(name, x, y, z) { O.an[name].position.set(x, y, z); }
function sweepOf(t0) {          // verification: bar crosses 0.4 s, edge lit 0.3 s after
  return { v: ramp(tt, t0, t0 + 0.4), e: tt >= t0 + 0.4 && tt <= t0 + 0.7 ? 1 : 0 };
}

function makeTok(family, content, size) {
  const g = makeDigestToken({ family, content, size });
  g.visible = false;
  ctx.scene.add(g);
  O.toks.push(g);
  return g;
}

function makeBar() {
  const g = new THREE.Group();
  P.box(O.mTi, 0.9, 0.15, 0.3, 0, 0.075, 0, g);
  P.box(O.mInk, 0.6, 0.02, 0.08, 0, 0.16, 0, g);
  g.visible = false;
  ctx.scene.add(g);
  return g;
}
function putBar(g, x, y, z, k = 1, sx = 1) {
  const on = k > 0.001;
  g.visible = on;
  if (!on) return;
  g.position.set(x, y, z);
  g.scale.set(sx * k, k, k);
}

function buildSlab(variant, kinds) {
  const s = makeProofSlab({ variant, pinKinds: kinds });
  s.visible = false;
  ctx.scene.add(s);
  O.slabs.push(s);
  const live = s.userData.pins.map((p) => p.children[0].material);
  s.userData.dim = P.makeDimmer(s, live);
  return s;
}

function hexOutline() {
  const m = P.mat(materials.gold());
  const r = new THREE.Mesh(O.hexGeo, m);
  r.visible = false;
  ctx.scene.add(r);
  r.userData.dim = P.makeDimmer(r);
  return r;
}
function flashOf(m, t0) { m.emissive.setHex(PALETTE.goldBright); m.emissiveIntensity = tt >= t0 && tt < t0 + 0.3 ? 0.5 : 0; }

export default {
  id: 'node',
  title: 'The node',
  beats: buildBeats(),

  build(c) {
    ctx = c;
    P = new Parts(ctx.scene);
    O = { an: {}, toks: [], slabs: [] };
    P.initMini();
    const sc = ctx.scene;

    for (const n of ANCHORS) { const a = new THREE.Object3D(); sc.add(a); O.an[n] = a; ctx.labels.anchor(n, a); }

    O.mTi = P.mat(materials.titanium());
    O.mCer = P.mat(materials.ceramic());
    O.mInk = P.mat(materials.ink());
    O.mGold = P.mat(materials.gold());
    O.mLine = P.lineMat();
    O.mGoldLit = P.mat(new THREE.MeshStandardMaterial({ color: PALETTE.goldBright, emissive: PALETTE.goldBright, emissiveIntensity: 1, roughness: 0.4 }));
    O.mStep = P.mat(materials.family('step'));
    O.hexGeo = P.geo(new THREE.TorusGeometry(0.55, 0.035, 4, 6));
    P.makeConnectors(40, O.mLine);

    // Node housing: dimmed ceramic top inside a titanium rim.
    const mHouse = P.mat(materials.ceramic());
    mHouse.color.set(PALETTE.ceramic).lerp(new THREE.Color(PALETTE.paperWarm), 0.7);
    P.box(O.mTi, 9.2, 0.24, 7.2, -6, 0.12, 0);
    P.box(mHouse, 9.0, 0.06, 7.0, -6, 0.27, 0);
    O.house = [];
    const mRecess = P.mat(materials.ink());
    for (const x of [-8, -4]) P.box(mRecess, 3.1, 0.02, 2.1, x, 0.31, 0);
    P.box(mRecess, 4, 0.02, 1.2, -6, 0.31, 2.6);
    O.neutral = P.box(P.mat(materials.ceramic()), 3.6, 0.06, 0.8, -6, 0.34, 2.6);
    O.neutral.material.color.set(PALETTE.ceramic).lerp(new THREE.Color(PALETTE.paperWarm), 0.6);

    // Key rack with three cards.
    O.rack = new THREE.Group();
    sc.add(O.rack);
    P.box(O.mTi, 7, 0.15, 0.15, -6, 4.2, -5.4, O.rack);
    P.box(O.mTi, 0.15, 4.2, 0.15, -9.3, 2.1, -5.4, O.rack);
    P.box(O.mTi, 0.15, 4.2, 0.15, -2.7, 2.1, -5.4, O.rack);
    const mLit = P.mat(materials.gold({ emissive: PALETTE.goldBright, emissiveIntensity: 0.45 }));
    const mDead = P.mat(materials.gold());
    [[-8.2, 3, mLit], [-6, 2, mLit], [-3.8, 1, mDead]].forEach(([x, n, m]) => {
      P.box(m, 1.6, 1.0, 0.08, x, 3.55, -5.4, O.rack);
      for (let k = 0; k < n; k++) P.box(O.mInk, 0.2, 0.12, 0.1, x + (k - (n - 1) / 2) * 0.4, 3.12, -5.4, O.rack);
    });

    // Slabs and tokens of the main line.
    O.leafG = buildSlab('leaf', KINDS_LEAF);
    O.nodeR = buildSlab('node', KINDS_NODE);
    O.nodeOut = buildSlab('node', KINDS_NODE);
    O.slabB = buildSlab('leaf', KINDS_LEAF);
    O.slabC = buildSlab('leaf', KINDS_LEAF);
    O.T = {
      progL: makeTok('program', 'prog:main', TOK), progR: makeTok('program', 'prog:main', TOK),
      preL: makeTok('state', 'state:init', TOK), postL: makeTok('state', 'state:0', TOK),
      preR: makeTok('state', 'state:0', TOK), postR: makeTok('state', 'state:1', TOK),
      b1: makeTok('state', 'state:0', TOK), b2: makeTok('state', 'state:1', TOK),
      c1: makeTok('state', 'state:1', TOK), c2: makeTok('state', 'state:final', TOK),
      oProg: makeTok('program', 'prog:main', 0.45), oPre: makeTok('state', 'state:init', 0.45), oPost: makeTok('state', 'state:1', 0.45),
      sIn: makeTok('state', 'state:1', TOK), sOut: makeTok('state', 'state:final', TOK),
      setRoot: makeTok('state', 'set:lr', 1.2), input: makeTok('input', 'input:main', 0.5),
    };
    // Tokens that other beats reuse (same family, other content).
    O.T.lSet = O.T.b1; O.T.rSet = O.T.b2; O.T.claim = O.T.sIn; O.T.shape = O.T.sOut;
    O.T.link1 = O.T.c1; O.T.link2 = O.T.c2; O.T.progK = O.T.progR;
    O.dimTok = O.toks.map((g) => P.makeDimmer(g));
    O.dimOf = new Map(O.toks.map((g, i) => [g, O.dimTok[i]]));

    // Equality outlines.
    O.oProg = hexOutline(); O.oState = hexOutline(); O.oSlice = hexOutline(); O.oSuffix = hexOutline();
    O.barOut = (() => { const m = P.mat(materials.gold()); const b = P.box(m, 1.0, 0.04, 0.4); b.visible = false; return b; })();
    O.barOut2 = (() => { const m = P.mat(materials.gold()); const b = P.box(m, 1.0, 0.04, 0.4); b.visible = false; return b; })();
    O.barOut3 = (() => { const m = P.mat(materials.gold()); const b = P.box(m, 1.0, 0.04, 0.4); b.visible = false; return b; })();

    // Rail, bars and the height counter.
    O.rail = P.box(O.mTi, 9, 0.06, 0.12, -5.5, 0.03, RAIL_Z);
    O.bars = { lo: makeBar(), hi: makeBar(), rlo: makeBar(), rhi: makeBar(), oLo: makeBar(), oHi: makeBar(), sStart: makeBar(), sEnd: makeBar(), suf: makeBar() };
    O.sweepBar = P.box(O.mGoldLit, 0.08, 0.3, 0.7, -9.4, 0.2, RAIL_Z);
    O.sweepBar.visible = false;
    O.hcol = new THREE.Group();
    sc.add(O.hcol);
    O.hcol.position.set(-9.5, 0.3, 5.3);
    {
      const im = new THREE.InstancedMesh(P.UB, O.mCer, 8);
      const m4 = new THREE.Matrix4();
      for (let k = 0; k < 8; k++) {
        const a = (k / 8) * Math.PI * 2;
        m4.compose(new THREE.Vector3(0.65 * Math.cos(a), 0, 0.65 * Math.sin(a)), new THREE.Quaternion().setFromAxisAngle(new THREE.Vector3(0, 1, 0), -a), new THREE.Vector3(0.28, 0.28, 0.28));
        im.setMatrixAt(k, m4);
      }
      O.hcol.add(im); O.hcolIm = im;
    }
    O.hruler = new THREE.Group();
    sc.add(O.hruler);
    P.box(O.mTi, 9.4, 0.1, 0.3, -3.3, 0.05, 5.3, O.hruler);
    for (const x of [-7.4, -6.6, -5.8, -2.6, -1.8, -1.0, -0.2, 0.6]) P.box(O.mLine, 0.04, 0.14, 0.1, x, 0.16, 5.3, O.hruler);
    P.box(O.mLine, 0.06, 0.34, 0.12, 1.4, 0.2, 5.3, O.hruler);
    for (const x of [-4.5, -4.1]) { const m = P.box(O.mLine, 0.05, 0.16, 0.5, x, 0.18, 5.3, O.hruler); m.rotation.y = rad(20); }

    // Fold block and cells.
    O.fold = new THREE.Group();
    sc.add(O.fold);
    O.foldMat = P.mat(materials.gold());
    P.box(O.foldMat, 1, 0.9, 1, 0, 0, 0, O.fold);
    for (const z of [-0.25, 0.25]) P.box(O.mInk, 0.12, 0.2, 0.3, -0.5, -0.1, z, O.fold);
    P.box(O.mInk, 0.12, 0.2, 0.3, 0.5, -0.1, 0, O.fold);
    O.cells = [0, 1, 2, 3].map(() => P.box(O.mGold, 1, 1, 1));
    O.cells.forEach((m) => { m.visible = false; });

    // Output caliper.
    O.cal = new THREE.Group();
    sc.add(O.cal);
    P.box(O.mTi, 0.2, 1.0, 2.4, -0.25, 0.5, 0, O.cal);
    P.box(O.mTi, 0.2, 1.0, 2.4, 3.05, 0.5, 0, O.cal);
    P.box(O.mTi, 3.4, 0.2, 2.4, 1.4, 0.1, 0, O.cal);

    // Tree minis, tree lines, ratio bars.
    O.leaf = LEAF_T.map(() => P.makeMini());
    O.nodeM = [0, 1, 2].map(() => P.makeMini(P.mat(materials.gold())));
    O.mv = P.makeMini();
    O.treeBreak = [0, 1].map((k) => { const m = P.box(O.mLine, 0.06, 0.06, 0.9, 10.05 + k * 0.4, 0.04, 1.35); m.rotation.y = rad(20); return m; });
    O.outl = [0, 1].map(() => { const g = new THREE.Group(); sc.add(g); P.box(O.mLine, 1.12, 0.03, 0.04, 0, 0, 0.38, g); P.box(O.mLine, 1.12, 0.03, 0.04, 0, 0, -0.38, g); P.box(O.mLine, 0.04, 0.03, 0.76, 0.55, 0, 0, g); P.box(O.mLine, 0.04, 0.03, 0.76, -0.55, 0, 0, g); return g; });
    O.ratioFrame = [2.4, 3.0, 3.6].map((z) => P.box(O.mTi, 9.5, 0.1, 0.3, 10.25, 0.05, z));
    const mFill = P.mat(materials.gold());
    O.ratioFill = [2.4, 3.0, 3.6].map((z) => P.box(mFill, 1, 0.12, 0.22, 5.5, 0.1, z));

    // Provers.
    O.prov = PROV.map(([x, z], k) => {
      const g = new THREE.Group(); sc.add(g); g.position.set(x, 0, z);
      const m = P.mat(materials.ceramic());
      P.box(m, 2.6, 0.1, 1.6, 0, 0.05, 0, g);
      P.box(m, 2.6, 1.2, 0.1, 0, 0.6, -0.75, g);
      P.box(m, 0.1, 1.2, 1.6, -1.25, 0.6, 0, g);
      P.box(m, 0.1, 1.2, 1.6, 1.25, 0.6, 0, g);
      const panel = P.box(m, 2.6, 1.2, 0.1, 0, 0.6, 0.75, g);
      panel.geometry = P.UB;
      const sw = P.box(O.mGoldLit, 0.08, 0.04, 0.8, 0, 0.5, 0, g);
      sw.visible = false;
      const base = m.color.clone();
      return { g, m, panel, sw, base };
    });
    O.thruFrame = P.box(O.mTi, 4, 0.06, 0.2, 10.25, 0.03, 5.9);
    O.thruFill = P.box(O.mGold, 1, 0.08, 0.14, 8.25, 0.07, 5.9);
    O.thruUnit = P.box(O.mLine, 0.05, 0.2, 0.3, 8.25 + 4 / 3, 0.1, 5.9);
    O.arrow = P.box(O.mLine, 9.5, 0.05, 0.06, 10.25, 0.04, 1.6);
    O.arrowHead = new THREE.Mesh(P.geo(new THREE.CylinderGeometry(0, 0.25, 0.5, 8)), O.mLine);
    O.arrowHead.rotation.z = -Math.PI / 2;
    sc.add(O.arrowHead);

    // Side tray and its objects.
    O.side = P.box(P.mat(materials.titanium()), 10, 0.1, 5.4, 10.25, 1.6, -1.2);
    O.hash = new THREE.Group();
    sc.add(O.hash);
    O.hashMat = P.mat(materials.gold());
    P.box(O.hashMat, 2.4, 1.0, 1.2, 0, 0, 0, O.hash);
    P.box(O.mInk, 0.1, 0.5, 0.5, -1.2, 0, 0, O.hash);
    P.box(O.mInk, 0.1, 0.5, 0.5, 1.2, 0, 0, O.hash);
    O.domain = P.box(O.mGold, 0.4, 0.4, 0.4);
    O.domain.visible = false;
    O.shk = new THREE.Group();
    sc.add(O.shk);
    O.miniKeys = P.box(O.mTi, 2.2, 0.08, 0.1, 0, 0.9, 0, O.shk);
    [-0.7, 0, 0.7].forEach((x) => P.box(O.mGold, 0.5, 0.35, 0.04, x, 0.6, 0, O.shk));
    O.dagSweep = P.box(O.mGoldLit, 0.1, 0.1, 0.1);
    O.dagSweep.visible = false;

    // Checkpoint store and tray.
    O.store = new THREE.Group();
    sc.add(O.store);
    O.store.position.set(-13.4, 0, 4.0);
    P.box(O.mTi, 3.6, 0.1, 2.6, 0, 0.05, 0, O.store);
    P.box(O.mTi, 3.6, 0.6, 0.1, 0, 0.3, -1.25, O.store);
    P.box(O.mTi, 3.6, 0.6, 0.1, 0, 0.3, 1.25, O.store);
    P.box(O.mTi, 0.1, 0.6, 2.6, -1.75, 0.3, 0, O.store);
    P.box(O.mTi, 0.1, 0.6, 2.6, 1.75, 0.3, 0, O.store);
    O.ckpt = new THREE.Group();
    sc.add(O.ckpt);
    P.box(O.mTi, 3.2, 0.1, 2.2, 0, 0, 0, O.ckpt);
    {
      const m4 = new THREE.Matrix4();
      const cells = new THREE.InstancedMesh(P.UB, O.mCer, 4);
      for (let k = 0; k < 4; k++) { m4.makeScale(0.4, 0.4, 0.4); m4.setPosition(-0.75 + k * 0.5, 0.25, -0.7); cells.setMatrixAt(k, m4); }
      const tiles = new THREE.InstancedMesh(P.UB, O.mCer, 3);
      for (let k = 0; k < 3; k++) { m4.makeScale(0.8, 0.12, 0.8); m4.setPosition(-1 + k, 0.11, 0.1); tiles.setMatrixAt(k, m4); }
      O.ckpt.add(cells, tiles);
    }
    P.box(O.mGold, 0.1, 0.1, 0.5, -0.8, 0.4, 0.78, O.ckpt).rotation.y = Math.PI / 2;
    O.ckptLink = O.ckpt.children[O.ckpt.children.length - 1];
    O.ckptLink.scale.set(0.5, 0.1, 0.1);

    // Witness tape slice: 4 frames.
    O.slice = new THREE.Group();
    sc.add(O.slice);
    {
      const m4 = new THREE.Matrix4();
      const plates = new THREE.InstancedMesh(P.UB, O.mCer, 4 + 24);
      const rules = new THREE.InstancedMesh(P.UB, O.mStep, 4);
      let n = 0;
      for (let f = 0; f < 4; f++) {
        const fx = (f - 1.5) * 1.0;
        m4.makeScale(0.9, 0.06, 1.2); m4.setPosition(fx, 0.03, 0); plates.setMatrixAt(n++, m4);
        m4.makeScale(0.3, 0.08, 0.5); m4.setPosition(fx, 0.1, 0); rules.setMatrixAt(f, m4);
        for (let q = 0; q < 3; q++) {
          m4.makeScale(0.18, 0.18, 0.18); m4.setPosition(fx - 0.3, 0.15, -0.4 + q * 0.4); plates.setMatrixAt(n++, m4);
          m4.makeScale(0.18, 0.18, 0.18); m4.setPosition(fx + 0.3, 0.15, -0.4 + q * 0.4); plates.setMatrixAt(n++, m4);
        }
      }
      O.slice.add(plates, rules);
    }

    O.childDim = P.makeDimmer(new THREE.Group());

    // Camera.
    const s = ctx.camera.shots;
    const IN = s.establish([-10, 1, 0], 30);
    const NODE = s.inspect([-6, 0, 0.6], 12, 'top');
    const RAIL = s.inspect([-5, 0, 4.4], 13, 'top');
    const PINS = s.establish([-6, 0.9, 1.0], 10);
    const OUTV = s.establish([-1.5, 0.8, 0.3], 13);
    const TREE = s.inspect([10.25, 0, 0.6], 13, 'top');
    const PROVV = s.establish([10.25, 0.8, 2.5], 14);
    const SIDE = s.inspect([10.25, 1.6, -1.2], 12, 'top');
    const CKPT = s.establish([-9.5, 0.8, 1.5], 15);
    const OUTW = s.establish([10, 1, 0], 30);
    ctx.camera.track([
      { t: S[0], pose: IN, dur: 2 },
      { t: S[1], pose: NODE, dur: 1.5 },
      { t: S[2], pose: RAIL, dur: 1.5 },
      { t: S[3], pose: PINS, dur: 1.5 },
      { t: S[5], pose: RAIL, dur: 1.5 },
      { t: S[6], pose: OUTV, dur: 1.5 },
      { t: S[8], pose: TREE, dur: 1.5 },
      { t: S[9], pose: PROVV, dur: 1.5 },
      { t: S[11], pose: NODE, dur: 1.5 },
      { t: S[12], pose: SIDE, dur: 1.5 },
      { t: S[14], pose: CKPT, dur: 1.5 },
      { t: S[16], pose: OUTV, dur: 1.5 },
      { t: S[16] + 6, pose: OUTW, dur: 1.5 },
    ]);
  },

  seek(t) {
    tt = t;
    const b = ctx.beatAt(t);
    bi = b.index;
    P.beginConnectors();
    const T = O.T;
    const cn = (ax, ay, az, bx, by, bz, k, w) => P.conn(ax, ay, az, bx, by, bz, k, w);
    const none = (g) => { g.visible = false; };

    // ---------------- rack, rail scaffolds
    O.rack.visible = true;

    // ---------------- children dim level
    const dimK = bi === 6 ? ph(6, 2, 2.6) : bi === 7 ? 1 : 0;

    // ---------------- leafG
    const kA = bi === 0 ? ph(0, 2, 3.6) : 1;
    {
      const x = mix(-20.8, SL[0], kA), y = mix(0.15, SL[1], kA), z = mix(0.9, SL[2], kA);
      putSlab(O.leafG, x, y, z, 1);
      const authed = bi > 1 || (bi === 1 && tt >= S[1] + 3.0);
      const sw1 = bi === 1 ? sweepOf(S[1] + 3.4) : { v: bi > 1 ? 1 : 0, e: 0 };
      const pre = bi >= 14;
      O.leafG.userData.setState({
        pins: authed ? FAM_LEAF : [], verify: sw1.v, edgeLit: pre ? 0.6 : sw1.e,
        ring: { show: bi === 14 || bi === 15 ? 1 : 0, folds: 0, closed: 0 },
      });
      O.leafG.userData.dim(dimK);
      if (bi >= 3 && bi <= 10 && bi !== 8 && bi !== 9) { /* in place */ }
      O.leafG.visible = bi <= 7 || bi >= 11;
      O.leafG.userData.pos = [x, y, z];
    }

    // ---------------- nodeR
    {
      const vis = bi <= 13 || bi === 16;
      const sc = bi === 16 ? ap(16, 0.2) : bi === 14 ? lv(14, 0.2) : bi === 15 ? 0 : 1;
      putSlab(O.nodeR, SR[0], SR[1], SR[2], vis ? Math.max(sc, 0) : 0);
      const authed = bi > 1 || (bi === 1 && tt >= S[1] + 3.0);
      const sw = bi === 1 ? sweepOf(S[1] + 3.8) : { v: 0, e: 0 };
      const folds = bi === 7 ? (tt < S[7] + 2.4 ? 1 : 0) : 1;
      O.nodeR.userData.setState({ pins: authed ? FAM_NODE : [], verify: sw.v, edgeLit: sw.e, ring: { show: 1, folds, closed: 0 } });
      O.nodeR.userData.dim(dimK);
    }

    // ---------------- stack slabs B and C (b0), B is the suffix in b15
    {
      const kB = bi === 0 ? ph(0, 3.6, 5.0) : 1;
      const arc = (k) => 1.8 * Math.sin(Math.PI * k);
      const bShow = bi === 0 && kB < 1;
      putSlab(O.slabB, mix(-20, 11.5, kB), mix(0.55, 0.3, kB) + arc(kB), mix(0, 0.8, kB), mix(1, MINI, kB));
      putSlab(O.slabC, mix(-19.2, 14, kB), mix(0.95, 0.3, kB) + arc(kB), mix(-0.9, 0.8, kB), mix(1, MINI, kB));
      O.slabB.visible = bShow;
      O.slabC.visible = bShow;
      O.slabB.userData.setState({ pins: [] });
      O.slabC.userData.setState({ pins: [] });
      // Suffix in b15 uses slab B.
      if (bi === 15) {
        const k = ph(15, 3.4, 4.6);
        putSlab(O.slabB, mix(-4, SR[0], k), mix(0.45, SR[1], k), mix(2.8, SR[2], k), ap(15, 2.0));
        O.slabB.userData.setState({ pins: FAM_LEAF });
      }
    }

    // ---------------- tokens over the children
    const flatNode = bi === 11;
    const children = bi >= 3 && bi <= 7;
    const dimTok = (g, k) => O.dimOf.get(g)(k);
    {
      const tp = bi === 0
        ? [SL[0] + (O.leafG.userData.pos[0] - SL[0]), 0]
        : [SL[0], 0];
      // b0: tokens ride with the genesis slab and the two others.
      const lx = O.leafG.userData.pos[0], ly = O.leafG.userData.pos[1] + 1.0, lz = O.leafG.userData.pos[2];
      const k0 = mix(0.5 / TOK, 1, kA);
      if (bi === 0) {
        putTok(T.preL, lx - 1.2, ly, lz, k0); putTok(T.postL, lx + 1.2, ly, lz, k0);
        const kB = ph(0, 3.6, 5.0);
        const kk = kB < 0.98 ? (0.5 / TOK) * mix(1, MINI, kB) : 0;
        const bx = O.slabB.position.x, by = O.slabB.position.y + 1.0 * mix(1, MINI, kB), bz = O.slabB.position.z;
        const cx = O.slabC.position.x, cy = O.slabC.position.y + 1.0 * mix(1, MINI, kB), cz = O.slabC.position.z;
        putTok(T.b1, bx - 1.2 * mix(1, MINI, kB), by, bz, kk); putTok(T.b2, bx + 1.2 * mix(1, MINI, kB), by, bz, kk);
        putTok(T.c1, cx - 1.2 * mix(1, MINI, kB), cy, cz, kk); putTok(T.c2, cx + 1.2 * mix(1, MINI, kB), cy, cz, kk);
        for (const g of [T.progL, T.progR, T.preR, T.postR]) none(g);
      } else {
        for (const g of [T.b1, T.b2, T.c1, T.c2]) none(g);
        if (children || bi === 11 || bi === 14 || bi === 15) {
          // default: three tokens over each child
        }
        none(T.progL); none(T.progR); none(T.preL); none(T.postL); none(T.preR); none(T.postR);
        if (children) {
          const y = 1.3;
          const t4 = bi >= 3 ? 1 : 0;
          putTok(T.preL, SL[0] - 1.2, y, 0, t4);
          putTok(T.postL, SL[0] + 1.2, y, 0, t4);
          putTok(T.progR, SR[0], y, 0, t4);
          putTok(T.postR, SR[0] + 1.2, y, 0, t4);
          putTok(T.progL, SL[0], y, 0, t4);
          putTok(T.preR, SR[0] - 1.2, y, 0, t4);
          if (bi === 3) {
            const k = ph(3, 2.0, 3.2);
            putTok(T.progR, mix(SR[0], SL[0], k), y, 0, k < 0.83 ? 1 : 0);
            const m = ph(3, 3.4, 4.0);
            putTok(T.progL, mix(SL[0], -6, m), y, 0, 1);
          } else if (bi >= 4) {
            none(T.progR);
            putTok(T.progL, -6, y, 0, 1);
          }
          if (bi === 4) {
            const k = ph(4, 2.0, 3.2);
            putTok(T.preR, mix(SR[0] - 1.2, SL[0] + 1.2, k), y, 0, k < 0.83 ? 1 : 0);
          } else if (bi >= 5) none(T.preR);
        }
        if (bi === 11) {
          putTok(T.progL, -6, 0.85, 0, 1, true);
          putTok(T.postR, SR[0] + 1.2, 0.85, 0, 1, true);
        }
        if (bi === 14) putTok(T.postL, SL[0] + 1.2, 1.3, 0, 1);
        if (bi === 15) {
          putTok(T.postL, SL[0] + 1.2, 1.3, 0, 1);
          const k = ph(15, 3.4, 4.6);
          const sx = mix(-4, SR[0], k);
          const e = ph(15, 3.8, 5.0);
          putTok(T.preR, mix(sx - 1.2, SL[0] + 1.2, e), 1.3, 0, ap(15, 2.0) > 0.99 && e < 0.83 ? 1 : 0);
          if (ap(15, 2.0) <= 0.99) putTok(T.preR, sx - 1.2, mix(0.45, 1.3, 1) , 2.8, ap(15, 2.0));
        }
        void tp;
      }
      // dim
      for (const g of [T.progL, T.progR, T.preL, T.postL, T.preR, T.postR]) dimTok(g, bi === 6 || bi === 7 ? dimK : 0);
      void flatNode;
    }

    // ---------------- equality outlines
    {
      const hexAt = (o, x, y, z, size, show, flashT) => {
        o.visible = show;
        o.position.set(x, y, z);
        o.scale.setScalar(size);
        flashOf(o.material, flashT);
        o.userData.dim(bi === 6 || bi === 7 ? dimK : 0);
      };
      const e3 = S[3] + 3.2, e4 = S[4] + 3.2;
      const prog = bi >= 3 && bi <= 7;
      const mp = bi === 3 ? ph(3, 3.4, 4.0) : bi > 3 ? 1 : 0;
      hexAt(O.oProg, mix(SL[0], -6, mp), 1.3, 0, TOK, prog && (bi > 3 || tt >= S[3] + 2.95), e3 - 0.2);
      hexAt(O.oState, SL[0] + 1.2, 1.3, 0, TOK, bi >= 4 && bi <= 7 && (bi > 4 || tt >= S[4] + 2.95), e4 - 0.2);
      // slice equality (b11) and suffix equality (b15)
      hexAt(O.oSlice, SR[0] + 1.2, 0.85, 0, TOK, false, 0);
      O.oSlice.rotation.x = 0;
      if (bi === 11) {
        const done = tt >= S[11] + 5.2 - 0.2;
        O.oSlice.visible = done;
        O.oSlice.rotation.x = -Math.PI / 2;
        flashOf(O.oSlice.material, S[11] + 5.2);
      } else O.oSlice.rotation.x = 0;
      hexAt(O.oSuffix, SL[0] + 1.2, 1.3, 0, TOK, bi === 15 && tt >= S[15] + 5.0 - 0.2, S[15] + 5.0);
    }

    // ---------------- connector from entry of leafG to exit of nodeR (b4) and rack connectors (b1)
    if (bi === 1) {
      const k = ph(1, 2.0, 3.0);
      cn(SL[0], 0.7, 0, -8.2, 3.0, -5.4, k, 0.06);
      cn(SR[0], 0.7, 0, -6, 3.0, -5.4, k, 0.06);
    } else if (bi === 2) {
      cn(SL[0], 0.7, 0, -8.2, 3.0, -5.4, 1, 0.06);
      cn(SR[0], 0.7, 0, -6, 3.0, -5.4, 1, 0.06);
    }
    if (bi === 4) cn(SL[0] - 1.2, 1.3, 0, SR[0] + 1.2, 1.3, 0, ph(4, 3.6, 4.8), 0.05);
    else if (bi === 5 || bi === 6 || bi === 7) cn(SL[0] - 1.2, 1.3, 0, SR[0] + 1.2, 1.3, 0, 1, 0.05);

    // ---------------- rail, bars, counter
    {
      const railOn = (bi >= 2 && bi <= 7) || bi === 11 || bi >= 14 ? (bi === 2 ? ap(2, 0.2) : 1) : 0;
      putBox(O.rail, 9, 0.06, 0.12, -5.5, 0.03, RAIL_Z, railOn);
      const B = O.bars;
      const pinX = (i, n, px) => px + (-1.3 + (i * 2.6) / (n - 1));
      const loX = -9.4, hiX = -6.5;
      let lo = 0, hi = 0;
      if (bi === 2) {
        const k1 = ph(2, 2.0, 3.2), k2 = ph(2, 2.3, 3.5);
        putBar(B.lo, mix(pinX(5, 7, SL[0]), loX, k1), mix(0.55, 0.0, k1), mix(1.15, RAIL_Z, k1), 1);
        putBar(B.hi, mix(pinX(6, 7, SL[0]), hiX, k2), mix(0.55, 0.0, k2), mix(1.15, RAIL_Z, k2), 1);
        lo = k1; hi = k2;
      } else if (bi > 2 && (bi <= 7 || bi === 11 || bi >= 14)) {
        putBar(B.lo, loX, 0, RAIL_Z, 1); putBar(B.hi, hiX, 0, RAIL_Z, 1);
      } else { none(B.lo); none(B.hi); }
      if (bi < 2) { none(B.lo); none(B.hi); }
      // interval line of the left child
      if (bi === 2) cn(loX + 0.45, 0.2, RAIL_Z - 0.3, hiX - 0.45, 0.2, RAIL_Z - 0.3, ph(2, 3.2, 3.8), 0.05);
      else if (bi > 2 && bi <= 7 || bi === 11) cn(loX + 0.45, 0.2, RAIL_Z - 0.3, hiX - 0.45, 0.2, RAIL_Z - 0.3, 1, 0.05);
      void lo; void hi;
      // sweep bar along the rail (b2 at 3.6)
      if (bi === 2) {
        const k = ramp(tt, S[2] + 3.6, S[2] + 4.0);
        O.sweepBar.visible = k > 0 && k < 1;
        O.sweepBar.position.x = mix(loX, hiX, k);
      } else O.sweepBar.visible = false;
      // right child bars
      const rloStart = -5.5, rhiX = -2.6;
      if (bi === 5) {
        const a = ap(5, 0.4);
        putBar(B.rhi, rhiX, 0, RAIL_Z + 0.5, a);
        const k = ph(5, 2.0, 3.2);
        putBar(B.rlo, mix(rloStart, hiX, k), 0.05, RAIL_Z + 0.5 * (1 - k), k < 0.83 ? a : 0);
        cn(mix(rloStart, hiX, k), 0.2, RAIL_Z + 0.25, rhiX, 0.2, RAIL_Z + 0.25, a, 0.05);
      } else if ((bi > 5 && bi <= 7) || bi === 11 || bi >= 14) {
        putBar(B.rhi, rhiX, 0, RAIL_Z, 1); none(B.rlo);
        cn(hiX, 0.2, RAIL_Z - 0.3, rhiX, 0.2, RAIL_Z - 0.3, 1, 0.05);
      } else { none(B.rhi); none(B.rlo); }
      if (bi === 15 || bi === 14) none(B.rhi);
      // height equality outline
      const eqDone = bi > 5 || (bi === 5 && tt >= S[5] + 3.0);
      const showEq = (bi >= 5 && bi <= 7) || bi === 11;
      putBox(O.barOut, 1.0, 0.04, 0.4, hiX, 0.0, RAIL_Z, showEq && eqDone ? 1 : 0);
      flashOf(O.barOut.material, S[5] + 3.0);
      // counter and ruler
      const hc = bi === 2 ? ap(2, 4.4) : (bi > 2 && bi <= 7) || bi === 11 ? 1 : 0;
      O.hcol.visible = hc > 0.001; O.hcol.scale.setScalar(Math.max(hc, 0.001));
      O.hruler.visible = hc > 0.001; O.hruler.scale.set(Math.max(hc, 0.001), 1, Math.max(hc, 0.001));
      O.hruler.position.set(0, 0, 0);
      if (bi === 2) cn(hiX, 0.2, RAIL_Z, -9.5, 0.3, 5.3, ph(2, 4.4, 5.2), 0.05);
      else if ((bi > 2 && bi <= 7) || bi === 11) cn(hiX, 0.2, RAIL_Z, -9.5, 0.3, 5.3, 1, 0.05);
    }

    // ---------------- output slab
    {
      let x = OUT[0], y = OUT[1], z = 0, s = 1, vis = bi >= 6 && !(bi >= 8 && bi <= 10);
      if (bi === 6) {
        const k = ph(6, 2.6, 4.0);
        x = mix(-6, OUT[0], ph(6, 2.6, 3.5) * 1 + 0 * k);
        x = mix(-6, OUT[0], phase(k, 0, 0.6));
        y = mix(1.6, OUT[1], phase(k, 0.6, 1));
        s = ap(6, 2.6);
      }
      let hand = 0;
      if (bi === 16) { hand = ph(16, 8.0, 10.4); x = mix(OUT[0], 20, hand); y = mix(OUT[1], 0.25, hand); }
      putSlab(O.nodeOut, x, y, z, vis ? s : 0);
      const pinsDone = bi > 6 ? 5 : bi === 6 ? (tt >= S[6] + 5.6 ? 5 : tt >= S[6] + 5.4 ? 3 : tt >= S[6] + 5.2 ? 2 : tt >= S[6] + 5.0 ? 1 : 0) : 0;
      const pc = [pinsDone >= 1 ? 'program' : null, pinsDone >= 2 ? 'state' : null, pinsDone >= 3 ? 'state' : null, null, null];
      const folds = bi < 7 ? 0 : bi === 7 ? (tt >= S[7] + 4.8 ? 1 : 0) : bi === 11 && tt >= S[11] + 6.0 ? 2 : bi > 11 ? 2 : 1;
      const lit = bi === 16 ? 0.4 : 0;
      O.nodeOut.userData.setState({ pins: pc, edgeLit: lit, ring: { show: 1, folds, closed: 0 } });
      const od = bi >= 11 && bi <= 15 ? 1 : bi === 16 ? 1 - ph(16, 0, 1.5) : 0;
      O.nodeOut.userData.dim(od);
      O.cal.visible = vis;
      O.cal.scale.setScalar(bi === 6 ? Math.max(ap(6, 2.0), 0.001) : 1);
      O.nodeOut.userData.pos = [x, y, z];
      // hovering pin tokens and bars
      const hx = (i) => x + (-1.3 + i * 0.65);
      const hy = 0.72, hz = 1.8;
      const prog = bi === 6 ? ph(6, 4.4, 5.0) : bi > 6 ? 1 : 0;
      const pre = bi === 6 ? ph(6, 4.6, 5.2) : bi > 6 ? 1 : 0;
      const post = bi === 6 ? ph(6, 4.8, 5.4) : bi > 6 ? 1 : 0;
      const k45 = 0.45 / 0.45;
      const fly = (g, k, fx, fy, fz, tx, ty, tz, scl) => putTok(g, mix(fx, tx, k), mix(fy, ty, k), mix(fz, tz, k), k > 0 ? scl * mix(TOK / 0.45, 1, k) : 0);
      const showOut = vis;
      if (showOut) {
        fly(T.oProg, prog, SL[0], 1.3, 0, hx(0), hy, z + hz, k45);
        fly(T.oPre, pre, SL[0] - 1.2, 1.3, 0, hx(1), hy, z + hz, k45);
        fly(T.oPost, post, SR[0] + 1.2, 1.3, 0, hx(2), hy, z + hz, k45);
        if (bi >= 11) T.oPost.userData.setContent(bi === 11 && tt < S[11] + 5.6 ? 'state:1' : 'state:final');
        else T.oPost.userData.setContent('state:1');
        if (bi < 6) { none(T.oProg); none(T.oPre); none(T.oPost); }
      } else { none(T.oProg); none(T.oPre); none(T.oPost); }
      for (const g of [T.oProg, T.oPre, T.oPost]) dimTok(g, od);
      const kl = bi === 6 ? ph(6, 5.0, 5.6) : bi > 6 ? 1 : 0;
      putBar(O.bars.oLo, mix(-9.4, hx(3), kl), mix(0.1, 0.62, kl), mix(RAIL_Z, z + hz, kl), showOut ? (kl > 0 ? 0.6 : 0) : 0);
      putBar(O.bars.oHi, mix(-2.6, hx(4), kl), mix(0.1, 0.62, kl), mix(RAIL_Z, z + hz, kl), showOut ? (kl > 0 ? 0.6 : 0) : 0);
      if (!showOut) { none(O.bars.oLo); none(O.bars.oHi); }
    }

    // ---------------- fold block, cells (b7)
    {
      const fo = bi === 7 ? ap(7, 0.2) : bi > 7 && bi <= 11 || bi >= 14 ? 1 : 0;
      const fOn = bi >= 7 && bi !== 8 && bi !== 9 && bi !== 10 && bi !== 12 && bi !== 13 ? (bi === 7 ? fo : 1) : 0;
      O.fold.visible = fOn > 0.001;
      O.fold.position.set(-1.6, 0.6, 0);
      O.fold.scale.setScalar(Math.max(fOn, 0.001));
      const pulse = bi === 7 && tt >= S[7] + 3.4 && tt < S[7] + 3.8 ? 0.8 : 0;
      O.foldMat.emissive.setHex(PALETTE.goldBright);
      O.foldMat.emissiveIntensity = pulse;
      const [c0, c1, c2, c3] = O.cells;
      c0.visible = c1.visible = c2.visible = c3.visible = false;
      if (bi === 7) {
        const sz = 0.3;
        const ra = ph(7, 2.0, 3.2), rb = ph(7, 2.2, 3.4), rm = ph(7, 2.4, 3.4);
        const cell = (m, k, fx, fy, fz, tx, ty, tz) => {
          if (k <= 0 || k >= 1) { m.visible = false; return; }
          m.visible = true; m.scale.set(sz, sz, sz);
          m.position.set(mix(fx, tx, k), mix(fy, ty, k) + 0.5 * Math.sin(Math.PI * k), mix(fz, tz, k));
        };
        cell(c0, ra, SL[0], 0.86, -0.25, -2.15, 0.5, -0.25);
        cell(c1, rb, SR[0], 0.86, 0.25, -2.15, 0.5, 0.25);
        cell(c2, rm, SR[0] + 0.31, 0.84, 0.63, -2.15, 0.5, 0.25);
        cell(c3, ph(7, 3.8, 4.8), -1.1, 0.5, 0, OUT[0] + 0.307, 0.74, 0.629);
        const kk = ph(7, 2.0, 3.2);
        cn(SL[0], 0.8, -0.25, -2.1, 0.8, -0.25, kk, 0.05);
        cn(SR[0], 0.8, 0.25, -2.1, 0.8, 0.25, ph(7, 2.2, 3.4), 0.05);
      }
    }

    // ---------------- tree minis, ratio, provers (b8 to b10)
    {
      const inTree = bi >= 8 && bi <= 10;
      const treeMiniOn = bi === 8 ? 1 : bi === 9 ? lv(9, 0) : bi === 10 ? 1 : 0;
      for (let k = 0; k < 4; k++) P.putMini(O.leaf[k], LEAF_T[k][0], 0.02, LEAF_T[k][1], inTree ? MINI : 0, 0, 0);
      // outlines
      O.outl[0].visible = bi === 8; O.outl[0].position.set(NODE_T[0][0], 0.03, NODE_T[0][1]); O.outl[0].scale.set(MINI * 3.14 / 1.12, 1, MINI * 2.14 / 0.76 * 0.76 / 2.14 * 2.14 / 2.14);
      O.outl[1].visible = bi === 8; O.outl[1].position.set(ROOT_T[0], 0.03, ROOT_T[1]); O.outl[1].scale.copy(O.outl[0].scale);
      O.outl[0].scale.set(MINI * 3.14 / 1.12, 1, MINI * 2.14 / 0.76);
      O.outl[1].scale.copy(O.outl[0].scale);
      O.ringOutlineAt = ROOT_T;
      const kMv = bi === 8 ? ph(8, 2.0, 3.6) : 1;
      // node minis
      if (bi === 8) {
        P.putMini(O.nodeM[0], NODE_T[0][0], 0.02, NODE_T[0][1], MINI, 1, 1);
        P.putMini(O.nodeM[1], NODE_T[1][0], 0.02, NODE_T[1][1], MINI, 1, 1);
        P.putMini(O.nodeM[2], 0, 0, 0, 0);
        P.putMini(O.mv, mix(NODE_T[0][0], ROOT_T[0], kMv), 0.06, mix(NODE_T[0][1], ROOT_T[1], kMv), MINI, 1, kMv < 0.5 ? 1 : 2);
      } else if (bi === 9) {
        const s = treeMiniOn;
        P.putMini(O.mv, ROOT_T[0], 0.06, ROOT_T[1], MINI * s, 1, 2);
        for (let k = 0; k < 3; k++) P.putMini(O.nodeM[k], 0, 0, 0, 0);
        // the middle prover holds the finished slab
        const pm = ph(9, 2.0, 2.8);
        P.putMini(O.nodeM[1], PROV[1][0], 0.12, PROV[1][1], MINI * ap(9, 2.0), 1, 1);
        void pm;
        P.putMini(O.nodeM[0], NODE_T[0][0], 0.02, NODE_T[0][1], MINI * lv(9, 0), 1, 1);
        if (O.nodeM[0].visible === false) { /* hidden */ }
      } else if (bi === 10) {
        const out = ph(10, 3.0, 4.4);
        const a0 = ap(10, 1.0);
        P.putMini(O.mv, 0, 0, 0, 0);
        P.putMini(O.nodeM[0], mix(PROV[0][0], NODE_T[0][0], out), 0.12 - 0.1 * out, mix(PROV[0][1], NODE_T[0][1], out), MINI * a0, 1, 1);
        P.putMini(O.nodeM[1], mix(PROV[1][0], ROOT_T[0], out), 0.12 - 0.06 * out, mix(PROV[1][1], ROOT_T[1], out), MINI, 1, 2);
        P.putMini(O.nodeM[2], mix(PROV[2][0], NODE_T[1][0], out), 0.12 - 0.1 * out, mix(PROV[2][1], NODE_T[1][1], out), MINI * a0, 1, 1);
      } else if (bi === 12) {
        // JOIN: two child sets and the joined set, on the side tray
        const r = ph(12, 0, 1.6);
        const a = ap(12, 2.0);
        P.putMini(O.nodeM[0], 7, 1.65, -0.2, 0.5 * a, 1, 1);
        P.putMini(O.nodeM[1], 7, 1.65, -2.2, 0.5 * a, 1, 1);
        P.putMini(O.nodeM[2], 12.7, 1.65, -1.2, 0.5 * clamp01((tt - S[12] - 5.0) / 0.4), 1, 1);
        P.putMini(O.mv, 0, 0, 0, 0);
        void r;
      } else if (bi === 13) {
        const sl = ph(13, 0, 1.6);
        const lx = mix(7, 5.4, sl);
        P.putMini(O.nodeM[0], lx, 1.65, -0.2 - 0.3 * sl, 0.4, 1, 1);
        P.putMini(O.nodeM[1], lx, 1.65, -2.2 + 0.1 * sl, 0.4, 1, 1);
        P.putMini(O.nodeM[2], mix(12.7, 7.4, sl), 1.65, -1.2 - 0.3 * sl, 0.5, 1, 1);
        // consumers and join slab on the right two thirds
        P.putMini(O.leaf[0], 12, 1.65, -0.4, 0.4 * ap(13, 0.2), 1, 1);
        P.putMini(O.leaf[1], 12, 1.65, 1.6, 0.4 * ap(13, 0.2), 1, 1);
        P.putMini(O.mv, 14, 1.65, 0.6, 0.4 * ap(13, 0.2), 1, 1);
      } else if (bi !== 8) {
        for (let k = 0; k < 3; k++) P.putMini(O.nodeM[k], 0, 0, 0, 0);
        P.putMini(O.mv, 0, 0, 0, 0);
      }
      if (bi === 13) { for (let k = 2; k < 4; k++) P.putMini(O.leaf[k], 0, 0, 0, 0); }
      if (bi === 12) { for (let k = 0; k < 4; k++) P.putMini(O.leaf[k], 0, 0, 0, 0); }
      if (bi === 13) { /* leaf[0] and leaf[1] set above */ } else if (bi !== 8 && bi !== 9 && bi !== 10) { for (let k = 0; k < 4; k++) P.putMini(O.leaf[k], 0, 0, 0, 0); }
      // tree lines
      if (inTree) {
        const L = (a, b2) => cn(a[0], 0.04, a[1] - 0.4, b2[0], 0.04, b2[1] + 0.4, 1, 0.04);
        const showNodes = bi === 8 || bi === 10;
        if (bi === 8 || bi === 10) {
          L(LEAF_T[0], NODE_T[0]); L(LEAF_T[1], NODE_T[0]); L(LEAF_T[2], NODE_T[1]); L(LEAF_T[3], NODE_T[1]);
        }
        if (bi === 8) { cn(NODE_T[0][0], 0.04, NODE_T[0][1] - 0.4, ROOT_T[0], 0.04, ROOT_T[1] + 0.4, 1, 0.04); cn(NODE_T[1][0], 0.04, NODE_T[1][1] - 0.4, ROOT_T[0], 0.04, ROOT_T[1] + 0.4, 1, 0.04); }
        if (bi === 10 && out10() >= 1) { cn(NODE_T[0][0], 0.04, NODE_T[0][1] - 0.4, ROOT_T[0], 0.04, ROOT_T[1] + 0.4, 1, 0.04); cn(NODE_T[1][0], 0.04, NODE_T[1][1] - 0.4, ROOT_T[0], 0.04, ROOT_T[1] + 0.4, 1, 0.04); }
        void showNodes;
      }
      for (const m of O.treeBreak) m.visible = inTree;
      // ratio bars
      const rOn = bi === 8 ? 1 : bi === 9 ? lv(9, 0) : 0;
      const grow = bi === 8 ? ph(8, 4.4, 6.0) : 1;
      O.ratioFrame.forEach((m, k) => { m.visible = rOn > 0.001; });
      O.ratioFill.forEach((m, k) => { const len = Math.max(9.5 * grow, 0.001); P.setBox(m, len, 0.12, 0.22, 5.5 + len / 2, 0.1, [2.4, 3.0, 3.6][k], rOn); });
      // provers
      const provOn = bi >= 9 && bi <= 10;
      O.prov.forEach((p, k) => {
        p.g.visible = provOn;
        const lit = bi === 9 ? (k === 1 ? 1 : 0) : bi === 10 ? (k === 1 ? 1 : ph(10, 0.2, 0.8)) : 0;
        p.m.color.copy(p.base).lerp(P.dimColor, 0.6 * (1 - lit));
        const down = bi === 9 ? (k === 1 ? ph(9, 2.0, 2.8) : 0) : bi === 10 ? (k === 1 ? 1 : ph(10, 0.5, 1.3)) : 0;
        p.panel.scale.set(2.6, Math.max(1.2 * (1 - down), 0.001), 0.1);
        p.panel.position.y = 0.6 * (1 - down) + 0.001;
        p.panel.visible = down < 0.999;
        const sv = bi === 9 ? (k === 1 ? ramp(tt, S[9] + 3.2, S[9] + 3.6) : 0) : bi === 10 ? ramp(tt, S[10] + 2.0, S[10] + 2.4) : 0;
        p.sw.visible = sv > 0 && sv < 1;
        p.sw.position.x = mix(-0.5, 0.5, sv);
      });
      // prover slab edge flash
      const fl = (m, t0) => { m.emissive.setHex(PALETTE.goldBright); m.emissiveIntensity = tt >= t0 + 0.4 && tt < t0 + 0.7 ? 0.8 : 0; };
      fl(O.nodeM[1].children[0].material, bi === 9 ? S[9] + 3.2 : S[10] + 2.0);
      fl(O.nodeM[0].children[0].material, S[10] + 2.0);
      fl(O.nodeM[2].children[0].material, S[10] + 2.0);
      if (bi === 10) {
        const k = ph(10, 0.4, 1.2);
        cn(PROV[0][0], 0.6, PROV[0][1] - 0.8, LEAF_T[0][0], 0.1, LEAF_T[0][1] + 0.4, k, 0.04);
        cn(PROV[0][0], 0.6, PROV[0][1] - 0.8, LEAF_T[1][0], 0.1, LEAF_T[1][1] + 0.4, k, 0.04);
        cn(PROV[1][0], 0.6, PROV[1][1] - 0.8, LEAF_T[1][0], 0.1, LEAF_T[1][1] + 0.4, k, 0.04);
        cn(PROV[1][0], 0.6, PROV[1][1] - 0.8, LEAF_T[2][0], 0.1, LEAF_T[2][1] + 0.4, k, 0.04);
        cn(PROV[2][0], 0.6, PROV[2][1] - 0.8, LEAF_T[2][0], 0.1, LEAF_T[2][1] + 0.4, k, 0.04);
        cn(PROV[2][0], 0.6, PROV[2][1] - 0.8, LEAF_T[3][0], 0.1, LEAF_T[3][1] + 0.4, k, 0.04);
      }
      // order arrow and throughput bar
      const ar = bi === 10 ? ph(10, 3.0, 4.4) : 0;
      O.arrow.visible = ar > 0.001;
      if (ar > 0.001) { const len = 9.5 * ar; O.arrow.scale.set(len, 0.05, 0.06); O.arrow.position.set(5.5 + len / 2, 0.04, 1.6); O.arrowHead.position.set(5.5 + len, 0.04, 1.6); }
      O.arrowHead.visible = ar > 0.001;
      const th = bi === 10 ? ph(10, 3.0, 5.0) : 0;
      const thOn = bi === 10 ? 1 : 0;
      O.thruFrame.visible = thOn > 0; O.thruUnit.visible = thOn > 0;
      const tl = Math.max(4 * (1.73 / 3) * th, 0.001);
      P.setBox(O.thruFill, tl, 0.08, 0.14, 8.25 + tl / 2, 0.07, 5.9, thOn);
    }

    // ---------------- b11: slice in the work socket
    {
      const on = bi === 11 || bi === 12 || bi === 13;
      let k = bi === 11 ? ph(11, 2.0, 3.4) : 1;
      O.slice.visible = on;
      O.slice.scale.setScalar(0.6);
      O.slice.position.set(-6, mix(0.5, 0.4, k) , mix(5.6, 2.6, k));
      O.neutral.visible = !(bi === 11 && tt > S[11] + 3.4) && !(bi >= 12 && bi <= 13);
      if (bi === 12 || bi === 13) O.slice.visible = false;
      if (bi === 11) {
        const sx = -6 - 1.2, ex = -6 + 1.2;
        const zS = O.slice.position.z, yS = 0.85;
        const e = ph(11, 4.0, 5.2);
        putTok(T.sIn, sx, yS, mix(zS, 0, e), (tt >= S[11] + 2.0) ? 1 : 0, true);
        if (e >= 0.83) T.sIn.visible = false;
        putTok(T.sOut, ex, yS, zS, tt >= S[11] + 2.0 ? 1 : 0, true);
        T.sIn.userData.setContent('state:1'); T.sOut.userData.setContent('state:final');
        // connector from the slice to the program token, start-height bar meets R.h_hi, new bar
        const c = ph(11, 5.6, 6.4);
        cn(-6, 0.85, zS, -6, 0.85, 0, c, 0.05);
        putBar(O.bars.sStart, mix(-5.2, -2.6, c), 0.0, mix(5.0, RAIL_Z, c), tt >= S[11] + 5.6 ? 1 : 0);
        putBar(O.bars.sEnd, -1.6, 0.0, RAIL_Z, c);
        cn(-1.6, 0.2, RAIL_Z, OUT[0] + 1.3, 0.5, 1.15, c, 0.05);
        if (O.bars.sStart.visible && c >= 1) O.bars.sStart.visible = false;
      } else {
        none(O.bars.sStart); none(O.bars.sEnd);
        if (!(bi === 12 || bi === 13)) { none(T.sIn); none(T.sOut); }
      }
    }

    // ---------------- b12, b13: side tray
    {
      const rise = bi === 12 ? ph(12, 0, 1.6) : bi === 13 ? 1 : bi === 14 ? 1 - ph(14, 0, 1.5) : 0;
      const sOn = bi >= 12 && bi <= 14;
      P.setBox(O.side, 10, 0.1, 5.4, 10.25, mix(0.05, 1.6, rise), -1.2, sOn ? (bi === 14 && rise < 0.05 ? 0 : 1) : 0);
      const hashOn = bi === 12;
      O.hash.visible = hashOn && tt >= S[12] + 1.6;
      O.hash.position.set(10.25, 2.15, -1.2);
      O.hashMat.emissive.setHex(PALETTE.goldBright);
      O.hashMat.emissiveIntensity = tt >= S[12] + 3.8 && tt < S[12] + 4.2 ? 0.8 : 0;
      const flatY = 1.9;
      const h0 = S[12] + 2.0;
      const mvIn = (t0, i) => clamp01((tt - (t0 + i * 0.3)) / 0.3);
      if (bi === 12 || bi === 13) {
        const sh = bi === 13 ? ph(13, 0, 1.6) : 0;
        // child set roots on their mini slabs
        const lx = mix(7, 5.4, sh);
        putTok(T.lSet, lx, flatY + 0.35, bi === 12 ? -0.2 : -0.5, ap(12, 2.0) * (bi === 12 ? 1 : 1) , true);
        putTok(T.rSet, lx, flatY + 0.35, bi === 12 ? -2.2 : -2.1, bi === 12 ? ap(12, 2.0) : 1, true);
        T.lSet.userData.setContent('set:l'); T.rSet.userData.setContent('set:r');
        if (bi === 13) { /* tokens stay with the left third */ }
      } else { none(T.lSet); none(T.rSet); }
      if (bi === 12) {
        // hash motion at h0 + 1.0: domain cell, l copy, r copy enter the -x slot in order
        const t0 = S[12] + 3.0;
        const queue = (i, sx, sz) => {
          const gather = ph(12, 3.0, 3.6);
          const toX = 10.25 - 1.2 - 0.9 + i * 0.0, tz = -1.2;
          const k = clamp01((tt - (t0 + 0.6 + i * 0.3)) / 0.3);
          return [mix(sx, 10.25 - 2.0 + i * 0.5, gather) + 0 * toX, mix(1.9, 2.15, gather), mix(sz, tz, gather), 1 - k];
        };
        const d = queue(0, 8.2, -0.8), l = queue(1, 7.2, -0.2), r = queue(2, 7.2, -2.2);
        const dOn = tt >= S[12] + 2.4 && d[3] > 0.001;
        P.setBox(O.domain, 0.4, 0.4, 0.4, 8.4 + (d[0] - 8.2) , d[1], d[2], dOn ? d[3] : 0);
        putTok(T.claim, l[0], l[1] + 0.2, l[2], tt >= S[12] + 2.4 ? l[3] : 0, true);
        putTok(T.shape, r[0], r[1] + 0.2, r[2], tt >= S[12] + 2.4 ? r[3] : 0, true);
        T.claim.userData.setContent('set:l'); T.shape.userData.setContent('set:r');
        const out = ph(12, 5.2, 6.0);
        putTok(T.setRoot, mix(11.45, 12.7, out), 1.9 + 0.1, -1.2, out > 0 ? mix(0.2, 1, out) : 0, true);
        T.setRoot.userData.setContent('set:lr');
        const c = ph(12, 3.0, 3.6);
        cn(7, 1.7, -0.2, 10.0, 1.9, -1.0, c, 0.04);
        cn(7, 1.7, -2.2, 10.0, 1.9, -1.4, c, 0.04);
      } else if (bi === 13) {
        const sh = ph(13, 0, 1.6);
        putTok(T.setRoot, mix(12.7, 7.4, sh), 2.0, -1.2 - 0.3 * sh, 0.5 / 1.2 * 1.2 / 1.2 * 2.4 / 2.4, true);
        T.setRoot.scale.setScalar(0.5);
        putTok(T.claim, 10, 1.9, 0.6, 0.8 / TOK * ap(13, 0.2), true);
        T.claim.userData.setContent('claim:shared');
        const sh2 = ap(13, 4.2);
        putTok(T.shape, 14, 1.9, -1.8, 0.8 / TOK * sh2, true);
        T.shape.userData.setContent('shape:dag');
        const c = ph(13, 1.0, 2.0);
        cn(10, 1.8, 0.6, 12, 1.8, -0.4, c, 0.04); cn(10, 1.8, 0.6, 12, 1.8, 1.6, c, 0.04);
        cn(12, 1.8, -0.4, 14, 1.8, 0.6, c, 0.04); cn(12, 1.8, 1.6, 14, 1.8, 0.6, c, 0.04);
        const sw = ramp(tt, S[13] + 2.0, S[13] + 3.6);
        O.dagSweep.visible = sw > 0 && sw < 1;
        const half = sw < 0.5 ? sw * 2 : (sw - 0.5) * 2;
        // along both branches in two legs
        O.dagSweep.position.set(sw < 0.5 ? mix(10, 12, half) : mix(12, 14, half), 1.95, sw < 0.5 ? mix(0.6, -0.4, half) : mix(-0.4, 0.6, half));
        O.dagSweep.scale.set(0.15, 0.15, 0.15);
      } else { O.dagSweep.visible = false; if (bi < 12 || bi > 13) { none(T.setRoot); } }
      if (bi < 11 || bi > 13) { none(T.claim); none(T.shape); }
      O.domain.visible = O.domain.visible && bi === 12;
      O.hash.visible = O.hash.visible && bi === 12;
      O.shk.visible = bi === 13;
      O.shk.position.set(14.2, 1.65, 3.0 - 0 - 0.0);
      O.shk.position.set(9.0, 1.65, -3.2);
    }

    // ---------------- b14, b15: checkpoint
    {
      const st = bi >= 14 && bi <= 15;
      O.store.visible = st;
      O.ckpt.visible = st;
      let x = -13.4, y = 0.5, z = 0.5;
      if (bi === 14) {
        const up = ph(14, 2.0, 2.6), sl = ph(14, 2.6, 3.4), dn = ph(14, 3.4, 3.8);
        y = mix(0.5, 1.0, up); z = mix(0.5, 4.0, sl); y = mix(y, 0.55, dn);
      } else if (bi === 15) {
        z = 4.0; y = mix(0.55, 0.85, ph(15, 2.0, 3.0));
      }
      O.ckpt.position.set(x, y, z);
      O.ckpt.visible = st;
      // tokens that belong on the tray
      const tr = (g, ox, oz, size) => putTok(g, x + ox, y + 0.5, z + oz, st ? size : 0);
      if (st) {
        tr(T.link1, -1.2, 0.75, 0.5 / TOK); tr(T.link2, -0.5, 0.75, 0.5 / TOK);
        T.link1.userData.setContent('state:link0'); T.link2.userData.setContent('state:link1');
        tr(T.progK, 0.3, 0.75, 0.5 / TOK); tr(T.input, 1.1, 0.75, 1);
        O.ckptLink.position.set(-0.85, 0.4, 0.78);
      } else { none(T.link1); none(T.link2); none(T.progK); none(T.input); }
      if (bi === 14) {
        cn(x, y + 0.4, z - 1.1, SL[0] + 1.2, 1.1, 0, ph(14, 0.4, 1.6), 0.05);
        cn(-13.4, 0.6, 4.0, -8.2, 3.0, -5.4, ph(14, 3.8, 4.8), 0.05);
      } else if (bi === 15) cn(-13.4, 0.6, 4.0, -8.2, 3.0, -5.4, 1, 0.05);
    }

    // ---------------- b15 suffix
    {
      if (bi === 15) {
        const a = ap(15, 2.0);
        const k = ph(15, 3.4, 4.6);
        putBar(O.bars.suf, mix(-5.2, -6.5, k), 0, mix(RAIL_Z + 0.9, RAIL_Z, k), a > 0.001 && k < 1 ? 1 : 0);
        putBox(O.barOut2, 1.0, 0.04, 0.4, -6.5, 0.0, RAIL_Z, k >= 1 ? 1 : 0);
        flashOf(O.barOut2.material, S[15] + 4.6);
      } else { none(O.bars.suf); O.barOut2.visible = false; }
      O.barOut3.visible = false;
    }

    // ---------------- connectors left over
    P.endConnectors();

    // ---------------- anchors
    placeAnchors();

    // ---------------- misc
    O.dagSweep.visible = O.dagSweep.visible && bi === 13;
  },

  dispose() {
    if (O) {
      for (const g of O.toks) disposeGroup(g, true);
      for (const s of O.slabs) s.userData.dispose();
      for (const g of [O.rack, O.hcol, O.hruler, O.fold, O.cal, O.hash, O.shk, O.store, O.ckpt, O.slice]) if (g) g.parent && g.parent.remove(g);
      for (const g of O.bars ? Object.values(O.bars) : []) g.parent && g.parent.remove(g);
    }
    if (P) P.disposeAll();
    ctx = null; P = null; O = null;
  },
};

function out10() { return ramp(tt, S[10] + 3.0, S[10] + 4.4); }

function putBox(m, w, h, d, x, y, z, s) { P.setBox(m, w, h, d, x, y, z, s); }

function placeAnchors() {
  const A = at;
  const lp = O.leafG.userData.pos;
  const ox = O.nodeOut.userData.pos[0];
  const pinX = (i) => ox + (-1.3 + i * 0.65);
  switch (bi) {
    case 0:
      A('aLeaf', lp[0], lp[1] + 0.4, lp[2]);
      A('aNodeR', SR[0], 0.7, 0);
      A('aRack', -6, 4.2, -5.4);
      A('aHouse', -6, 0.3, 3.2);
      break;
    case 1: {
      const second = tt >= S[1] + 3.8;
      const cx = second ? SR[0] : SL[0];
      A('aActive', cx, 0.7, -1.0);
      A('aPins', cx, 0.6, 1.5);
      A('aKey', second ? -5.0 : -8.1, 1.5, -2.8);
      A('aNeutral', -6, 0.4, 2.6);
      break;
    }
    case 2:
      A('aHPre', -7.5, 0.6, 1.5);
      A('aSteps', -6.3, 0.6, 1.5);
      A('aHLo', -9.4, 0.3, RAIL_Z - 0.2);
      A('aHHi', -6.5, 0.3, RAIL_Z - 0.2);
      A('aCounter', -9.5, 0.6, 4.8);
      A('aRuler', 0.2, 0.3, 5.1);
      break;
    case 3:
      A('aHouse', -6, 0.3, -3.3);
      A('aLeft', SL[0], 1.3, 0);
      A('aRight', SR[0], 1.3, 0);
      A('aProg', -6, 1.3, 0);
      break;
    case 4:
      A('aPost', -7.5, 1.3, 0); A('aPre', -4.6, 1.3, 0); A('aEqual', -6.8, 1.3, 0);
      break;
    case 5:
      A('aLHi', -7.7, 0.2, RAIL_Z + 0.3); A('aRLo', -5.0, 0.2, RAIL_Z + 1.0); A('aEqH', -6.5, 0.2, RAIL_Z - 0.4);
      break;
    case 6:
      for (let i = 0; i < 5; i++) A('aPin' + i, pinX(i), 0.75, 1.8);
      break;
    case 7:
      A('aConns', -3.2, 0.8, 0.3); A('aFold', -1.6, 1.05, 0); A('aRing', OUT[0] - 0.5, 0.75, -0.5); A('aJoint', OUT[0], 0.7, 0.7);
      break;
    case 8:
      A('aFootprint', NODE_T[0][0], 0.1, NODE_T[0][1] + 0.5);
      A('aRingOutline', ROOT_T[0], 0.1, ROOT_T[1] - 0.2);
      A('aRSteps', 12.5, 0.1, 2.25); A('aRLeaf', 12.5, 0.1, 2.85); A('aRNode', 12.5, 0.1, 3.45);
      A('aRLinear', 15.2, 0.1, 3.0);
      break;
    case 9:
      A('aProver', PROV[1][0], 1.2, PROV[1][1]);
      A('aTime', PROV[1][0] - 1.6, 0.5, PROV[1][1] + 0.9);
      A('aThreads', PROV[1][0], 0.5, PROV[1][1] + 1.0);
      A('aMem', PROV[1][0] + 1.6, 0.5, PROV[1][1] + 0.9);
      break;
    case 10:
      A('aProvers', PROV[1][0], 1.2, PROV[1][1]);
      A('aReady', 6.5, 0.2, 0.8);
      A('aOrder', 13.5, 0.2, 1.6);
      A('aThru', 10.25, 0.2, 5.9);
      break;
    case 11:
      A('aHouse', -6, 0.3, -3.3); A('aPrior', SL[0], 0.7, -1.0); A('aNew', -6, 0.5, 3.0);
      A('aProg', -6, 0.85, -0.4); A('aPost', SR[0] + 1.2, 0.85, -0.6); A('aOutH', -1.6, 0.2, RAIL_Z - 0.3);
      break;
    case 12:
      A('aJoin', 10.25, 2.7, -1.2); A('aDomain', 8.2, 2.1, -0.8);
      A('aL', 7, 1.9, -0.8); A('aR', 7, 1.9, -2.8); A('aSetRoot', 12.7, 2.1, -1.8);
      break;
    case 13:
      A('aClaim', 10, 1.9, 0.2); A('aCons', 12, 1.9, 1.0); A('aDag', 12, 1.7, 2.2); A('aShape', 14, 1.9, -2.2); A('aKeys', 9.0, 1.7, -3.2);
      break;
    case 14:
      A('aStore', -11.6, 0.7, 5.2);
      A('aCkpt', -13.4, 0.9, 3.0);
      A('aPrefix', SL[0], 0.7, 1.0);
      A('aWit', -15.2, 0.6, 3.9);
      A('aPostP', SL[0] + 1.2, 1.3, 0);
      A('aKeySet', -11, 1.5, -0.8);
      break;
    case 15:
      A('aPrefix', SL[0], 0.7, 1.0);
      A('aSuffix', SR[0], 0.7, 1.0);
      A('aPostP', SL[0] + 1.2, 1.3, 0); A('aPreS', SR[0] - 1.2, 1.3, 0);
      A('aSavedH', -6.5, 0.2, RAIL_Z - 0.4); A('aSame', -6, 0.7, -1.3);
      break;
    default:
      A('aRoot', ox, 0.5, -1.0);
      A('aOuter', ox, 0.3, 2.3);
      A('aRing', ox - 2.4, 0.7, -0.4);
      A('aJoint', ox + 2.4, 0.7, 0.8);
  }
}
