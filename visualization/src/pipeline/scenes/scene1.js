// Scene 1: The program. Source sheet and input value, the press that makes the ROM rack,
// the program digest, the canonical value digest, the input commitment, the fixed key handoff.
// Every pose is a pure function of the scene time t. All motions are written in absolute scene
// time (beat start plus local seconds), so an earlier motion has finished when a later beat runs.
import * as THREE from 'three/webgpu';
import { PALETTE, FAMILY, materials, makeDigestToken, phase, ramp, lerp, clamp01 } from '../theme.js';
import { ident } from '../labels.js';
import { TEXT } from './scene1/text.js';

// ---- timeline -----------------------------------------------------------------------------
const B = [0, 22, 44, 66, 88, 117];
let t = 0;
const ph = (bi, a, b) => phase(t, B[bi] + a, B[bi] + b);
const rp = (bi, a, b) => ramp(t, B[bi] + a, B[bi] + b);
const pulse = (bi, a, b, f = 0.4) => Math.min(rp(bi, a, a + f), 1 - rp(bi, b - f, b));
const within = (bi, a, b) => (t >= B[bi] + a && t < B[bi] + b ? 1 : 0);
const after = (bi, a) => (t >= B[bi] + a ? 1 : 0);

// ---- scratch ------------------------------------------------------------------------------
const _m = new THREE.Matrix4();
const _p = new THREE.Vector3();
const _s = new THREE.Vector3();
const _q0 = new THREE.Quaternion();
const _qBreak = new THREE.Quaternion().setFromAxisAngle(new THREE.Vector3(0, 1, 0), (20 * Math.PI) / 180);
const _v = [0, 0, 0];

let ctx = null;
let o = null;
let owned = [];

const own = (x) => { owned.push(x); return x; };
const mesh = (geo, mat) => new THREE.Mesh(geo, mat);
const inst = (geo, mat, n) => { const im = new THREE.InstancedMesh(geo, mat, n); im.frustumCulled = false; return im; };

function setInst(im, i, x, y, z, sx, sy, sz, q = _q0) {
  _m.compose(_p.set(x, y, z), q, _s.set(sx, sy, sz));
  im.setMatrixAt(i, _m);
}
function setBox(m, x, y, z, w, h, d) {
  m.position.set(x, y, z);
  m.scale.set(w, h, d);
}
function place(obj, x, y, z, s) {
  obj.position.set(x, y, z);
  obj.scale.setScalar(Math.max(s, 0.001));
  obj.visible = s > 0.002;
}
// Swaps between a plain and a lit material. Only one object per material is lit at a time.
function useLit(m, plain, lit, amount) {
  if (amount > 0.001) { m.material = lit; lit.emissiveIntensity = amount; } else m.material = plain;
}

// ---- wires: thin boxes in one instanced mesh, each polyline grows from its first point ------
class Wires {
  constructor(mat) { this.mat = mat; this.polys = []; this.n = 0; this.mesh = null; }
  add(pts) {
    const p = { first: this.n, segs: [], len: 0 };
    for (let i = 0; i + 1 < pts.length; i++) {
      const a = pts[i], b = pts[i + 1];
      const l = Math.abs(b[0] - a[0]) + Math.abs(b[1] - a[1]) + Math.abs(b[2] - a[2]);
      p.segs.push({ a, b, l, s0: p.len });
      p.len += l;
    }
    this.n += p.segs.length;
    this.polys.push(p);
    return p;
  }
  finish(geo) { this.mesh = inst(geo, this.mat, this.n); return this.mesh; }
  set(p, g) {
    const L = p.len * clamp01(g);
    for (let j = 0; j < p.segs.length; j++) {
      const sg = p.segs[j];
      const f = clamp01((L - sg.s0) / sg.l);
      const i = p.first + j;
      if (f <= 0.0001) { setInst(this.mesh, i, 0, 0, 0, 0, 0, 0); continue; }
      const dx = sg.b[0] - sg.a[0], dy = sg.b[1] - sg.a[1], dz = sg.b[2] - sg.a[2];
      const T = 0.06;
      const sx = dx !== 0 ? Math.abs(dx) * f + T : T;
      const sy = dy !== 0 ? Math.abs(dy) * f + T : 0.04;
      const sz = dz !== 0 ? Math.abs(dz) * f + T : T;
      setInst(this.mesh, i, sg.a[0] + (dx * f) / 2, sg.a[1] + (dy * f) / 2, sg.a[2] + (dz * f) / 2, sx, sy, sz);
    }
  }
  commit() { this.mesh.instanceMatrix.needsUpdate = true; }
}

// ---- hash motion (fixed form, 3.0 s) ---------------------------------------------------------
// Writes the item position into _v and returns [k1, k2]: k1 = progress to the file, k2 = progress into the slot.
const _k = [0, 0];
function flow(tl, n, i, A, F, S) {
  const d = Math.min(0.3, 1.2 / n);
  const k1 = phase(tl, 0, 0.6);
  const k2 = ramp(tl, 0.6 + i * d, 0.6 + i * d + d);
  if (k2 <= 0) { for (let c = 0; c < 3; c++) _v[c] = lerp(A[c], F[c], k1); }
  else { for (let c = 0; c < 3; c++) _v[c] = lerp(F[c], S[c], k2); }
  _k[0] = k1; _k[1] = k2;
  return _k;
}
const glowOf = (tl) => (tl <= 1.8 || tl >= 2.2 ? 0 : 0.8 * Math.min((tl - 1.8) / 0.2, (2.2 - tl) / 0.2));

// ---- layout constants ---------------------------------------------------------------------------
const SHEET = [-12.5, 0.04, -0.5];
const INP = [-12.5, 0.3, 4.4];
const PRESS = [-7.5, -0.5];
const ROM0 = [-2.5, 1.3, -1.5];
const SHELF_Y = [0.95, 0.5, 0.05, -0.4, -0.85];
const HASH_P = [3.5, -1.5];
const HASH_V = [3.5, 2.5];
const HASH_I = [9.5, 2.5];
const TOK_P = [5.9, 0.7, -1.5];
const TOK_V = [5.9, 0.7, 2.5];
const TOK_I = [11.9, 0.7, 2.5];
const COLLAR_X = [7.0, 9.2, 11.4];
const LANE_Z = 4.6;
const SHK = [5.0, 0.2, LANE_Z];
const CARD_X = [1.8, 4.0, 6.2];
const CARRIER_PT = [0, 0.4, 5.8];
const SLOT_V = [2.3, 0.5, 2.5];
const SLOT_P = [2.3, 0.5, -1.5];
const SLOT_I = [8.3, 0.5, 2.5];
// sheet bars: slot, indent, length. Slot 2 is the gap between lift and main.
const BARS = [[0, 0, 2.6], [1, 0.4, 1.6], [3, 0, 3.2], [4, 0.4, 2.4], [5, 0.4, 1.8], [6, 0.4, 2.8], [7, 0.4, 2.0], [8, 0.8, 1.2], [9, 0, 2.6], [10, 0.4, 2.0]];
const barZ = (slot) => -2.3 + 0.43 * slot;
const barX = (b) => -1.7 + b[1] + b[2] / 2;
const LIFT = [0, 1];
const COND = [7, 9, 10];

// ---- the scene -------------------------------------------------------------------------------------
export default {
  id: 'program',
  title: 'The program',
  beats: [
    {
      id: TEXT[0].id, title: TEXT[0].title, duration: TEXT[0].duration, narration: TEXT[0].narration,
      labels: [
        { text: 'Moriarty source', anchor: 'sheetLabel', kind: 'program', dy: 1.6, from: 3.2 },
        { text: 'Input value', anchor: 'inpLabel', kind: 'input', dy: 0.9, from: 3.2 },
        { text: 'Named call', anchor: 'liftLabel', kind: 'program', dy: 1.0, from: 5.0 },
        { text: 'Conditional', anchor: 'condLabel', kind: 'program', dy: 0.1, from: 9.0 },
      ],
    },
    {
      id: TEXT[1].id, title: TEXT[1].title, duration: TEXT[1].duration, narration: TEXT[1].narration,
      labels: [
        { text: 'Compiler', anchor: 'pressLabel', kind: 'neutral', dy: 0.5 },
        { text: 'Program image', anchor: 'romLabel', kind: 'program', dy: 0.6, from: 5.0 },
        { text: 'Code tables', anchor: 'tablesA', kind: 'program', dy: 0.4, from: 6.0 },
        { text: 'Types and constants', anchor: 'tablesB', kind: 'program', dy: 0.4, from: 7.0 },
      ],
    },
    {
      id: TEXT[2].id, title: TEXT[2].title, duration: TEXT[2].duration, narration: TEXT[2].narration,
      labels: [
        { text: 'Poseidon', anchor: 'hashPLabel', kind: 'state', dy: 0.5, from: 1.0 },
        { text: 'Program image', anchor: 'romLabel', kind: 'program', dy: 0.6 },
        { text: ident('prog_d'), anchor: 'tokPLabel', kind: 'program', dy: 0.1, from: 5.0 },
      ],
    },
    {
      id: TEXT[3].id, title: TEXT[3].title, duration: TEXT[3].duration, narration: TEXT[3].narration,
      labels: [
        { text: 'Canonical value', anchor: 'cellsLabel', kind: 'input', dy: 0.7, from: 3.3, to: 6.2 },
        { text: ident('sherlock-value/1'), anchor: 'hashVLabel', kind: 'state', dy: 0.5, from: 1.5 },
        { text: ident('val_d'), anchor: 'tokVLabel', kind: 'input', dy: 0.1, from: 6.5 },
        { text: 'Equal value', anchor: 'eqLabel', kind: 'state', dy: 0.9, from: 10.3 },
      ],
    },
    {
      id: TEXT[4].id, title: TEXT[4].title, duration: TEXT[4].duration, narration: TEXT[4].narration,
      labels: [
        { text: ident('SHK_IN'), anchor: 'shkLabel', kind: 'state', dy: 0.5, from: 2.6 },
        { text: ident('val_d'), anchor: 'tokVLabel', kind: 'input', dy: 0.1 },
        { text: ident('fuel_lo'), anchor: 'collar0', kind: 'input', dy: 0.5, from: 2.6 },
        { text: ident('fuel_hi'), anchor: 'collar1', kind: 'input', dy: 0.5, from: 2.6 },
        { text: ident('stack_limit'), anchor: 'collar2', kind: 'input', dy: 0.5, from: 2.6 },
        { text: ident('input_d'), anchor: 'tokILabel', kind: 'input', dy: 0.1, from: 9.0 },
      ],
    },
    {
      id: TEXT[5].id, title: TEXT[5].title, duration: TEXT[5].duration, narration: TEXT[5].narration,
      labels: [
        { text: 'Leaf key', anchor: 'card0', kind: 'state', dy: 0.2, from: 2.4 },
        { text: 'Node key', anchor: 'card1', kind: 'state', dy: 1.3, from: 2.4 },
        { text: 'Decider key', anchor: 'card2', kind: 'state', dy: 0.2, from: 2.4 },
        { text: 'Program image', anchor: 'romLabel', kind: 'program', dy: 0.6 },
      ],
    },
  ],

  build(c) {
    ctx = c;
    owned = [];
    o = {};
    const root = ctx.scene;

    // geometry and materials
    const UB = own(new THREE.BoxGeometry(1, 1, 1));
    const TORUS = own(new THREE.TorusGeometry(0.44, 0.025, 8, 36));
    const mCeramic = own(materials.ceramic());
    const mInk = own(materials.ink());
    const mProg = own(materials.family('program'));
    const mProgLit = own(materials.family('program', { lit: 1 }));
    const mInput = own(materials.family('input'));
    const mTi = own(materials.titanium());
    const mTiLit = own(materials.titanium());
    mTiLit.emissive = new THREE.Color(PALETTE.goldBright);
    const mGold = own(materials.gold());
    const mGoldLit = own(materials.gold());
    mGoldLit.emissive = new THREE.Color(PALETTE.goldBright);
    const mSweep = own(materials.gold());
    mSweep.color.setHex(PALETTE.goldBright);
    mSweep.emissive = new THREE.Color(PALETTE.goldBright);
    mSweep.emissiveIntensity = 1;
    const mFloor = own(materials.paper());
    const mLine = own(new THREE.MeshBasicMaterial({ color: PALETTE.onSurface }));
    Object.assign(o, { UB, mCeramic, mInk, mProg, mProgLit, mInput, mTi, mTiLit, mGold, mGoldLit });

    const wires = new Wires(mLine);
    o.wires = wires;

    // --- source sheet, its bars, the input cube
    const sheet = new THREE.Group();
    const sheetPlate = mesh(UB, mCeramic);
    setBox(sheetPlate, 0, 0, 0, 4.0, 0.08, 5.4);
    sheet.add(sheetPlate);
    const barSet = (slots) => BARS.filter((b) => slots.includes(b[0]));
    const mkBars = (list, mat) => {
      const im = inst(UB, mat, list.length);
      list.forEach((b, i) => setInst(im, i, barX(b), 0.07, barZ(b[0]), b[2], 0.06, 0.22));
      im.instanceMatrix.needsUpdate = true;
      sheet.add(im);
      return im;
    };
    o.liftBars = mkBars(barSet(LIFT), mProg);
    o.condBars = mkBars(barSet(COND), mProg);
    mkBars(BARS.filter((b) => !LIFT.includes(b[0]) && !COND.includes(b[0])), mProg);
    sheet.position.set(...SHEET);
    root.add(sheet);
    o.sheet = sheet;

    const copy = new THREE.Group();
    const copyPlate = mesh(UB, mCeramic);
    setBox(copyPlate, 0, 0, 0, 4.0, 0.08, 5.4);
    copy.add(copyPlate);
    const copyBars = inst(UB, mProg, BARS.length);
    BARS.forEach((b, i) => setInst(copyBars, i, barX(b), 0.07, barZ(b[0]), b[2], 0.06, 0.22));
    copyBars.instanceMatrix.needsUpdate = true;
    copy.add(copyBars);
    root.add(copy);
    o.copy = copy;

    o.inp = mesh(UB, mInput);
    root.add(o.inp);
    o.cells = inst(UB, mInput, 4);
    root.add(o.cells);

    // --- compiler press
    const pBase = mesh(UB, mCeramic);
    setBox(pBase, PRESS[0], 0.25, PRESS[1], 3.0, 0.5, 5.6);
    o.platen = mesh(UB, mCeramic);
    o.rim = inst(UB, mTi, 3);
    root.add(pBase, o.platen, o.rim);

    // --- ROM rack
    const rom = new THREE.Group();
    o.frame = inst(UB, mTi, 12);
    {
      let k = 0;
      for (const sx of [-1, 1]) for (const sz of [-1, 1]) setInst(o.frame, k++, sx * 1.45, 0, sz * 0.75, 0.1, 2.6, 0.1);
      for (const sy of [-1, 1]) for (const sz of [-1, 1]) setInst(o.frame, k++, 0, sy * 1.25, sz * 0.75, 3.0, 0.1, 0.1);
      for (const sy of [-1, 1]) for (const sx of [-1, 1]) setInst(o.frame, k++, sx * 1.45, sy * 1.25, 0, 0.1, 0.1, 1.6);
      o.frame.instanceMatrix.needsUpdate = true;
    }
    rom.add(o.frame);
    o.shelves = [];
    for (let i = 0; i < 5; i++) {
      const s = mesh(UB, mProg);
      setBox(s, 0, 0, 0, 2.6, 0.08, 1.2);
      rom.add(s);
      o.shelves.push(s);
    }
    o.cellsRom = inst(UB, mCeramic, 30);
    rom.add(o.cellsRom);
    rom.position.set(...ROM0);
    root.add(rom);
    o.rom = rom;
    o.romCopies = inst(UB, mProg, 5);
    root.add(o.romCopies);
    // rom2: the same rack at scale 0.6 (frame, shelves, cells)
    const rom2 = new THREE.Group();
    const frame2 = inst(UB, mTi, 12);
    {
      let k = 0;
      for (const sx of [-1, 1]) for (const sz of [-1, 1]) setInst(frame2, k++, sx * 1.45, 0, sz * 0.75, 0.1, 2.6, 0.1);
      for (const sy of [-1, 1]) for (const sz of [-1, 1]) setInst(frame2, k++, 0, sy * 1.25, sz * 0.75, 3.0, 0.1, 0.1);
      for (const sy of [-1, 1]) for (const sx of [-1, 1]) setInst(frame2, k++, sx * 1.45, sy * 1.25, 0, 0.1, 0.1, 1.6);
      frame2.instanceMatrix.needsUpdate = true;
    }
    const shelves2 = inst(UB, mProg, 5);
    const cells2 = inst(UB, mCeramic, 30);
    for (let i = 0; i < 5; i++) {
      setInst(shelves2, i, 0, SHELF_Y[i], 0, 2.6, 0.08, 1.2);
      for (let j = 0; j < 6; j++) setInst(cells2, i * 6 + j, -1.0 + 0.4 * j, SHELF_Y[i] + 0.165, 0, 0.25, 0.25, 0.25);
    }
    shelves2.instanceMatrix.needsUpdate = true;
    cells2.instanceMatrix.needsUpdate = true;
    rom2.add(frame2, shelves2, cells2);
    root.add(rom2);
    o.rom2 = rom2;

    // --- hash blocks
    const mkHash = (xz) => {
      const block = mesh(UB, mGold);
      const slots = inst(UB, mInk, 2);
      root.add(block, slots);
      return { block, slots, x: xz[0], z: xz[1] };
    };
    o.hashP = mkHash(HASH_P);
    o.hashV = mkHash(HASH_V);
    o.hashI = mkHash(HASH_I);

    // --- tokens
    const mkTok = (family, content, size) => {
      const k = makeDigestToken({ family, content, size });
      root.add(k);
      return k;
    };
    o.tokP = mkTok('program', 'prog:main', 1.2);
    o.tokV = mkTok('input', 'val:input', 1.2);
    o.tokI = mkTok('input', 'input:main', 1.2);
    o.tokP2 = mkTok('program', 'prog:other', 0.8);
    o.travP = mkTok('program', 'prog:main', 1.2);
    o.travI = mkTok('input', 'val:input', 1.2);
    o.valL = mkTok('input', 'val:struct', 0.8);
    o.valR = mkTok('input', 'val:struct', 0.8);

    // --- connectors
    o.wTokP = wires.add([[-0.9, 0.04, -1.5], [-0.9, 0.04, -3.3], [5.9, 0.04, -3.3], [5.9, 0.04, -1.5]]);
    o.wDrop = CARD_X.map((x) => wires.add([[x, 3.05, -5.4], [x, 0.18, -5.4]]));
    o.wMain = wires.add([[-1.5, 0.18, -5.4], [9.0, 0.18, -5.4]]);
    o.wBranchRom = wires.add([[-1.5, 0.18, -5.4], [-1.5, 0.18, -2.4]]);
    o.wBranchRom2 = wires.add([[9.0, 0.18, -5.4], [9.0, 0.18, -3.75]]);
    const ty = 0.13;
    o.wIn = [
      wires.add([[7.2, ty, -4.1], [7.2, ty, -3.55], [8.2, ty, -3.55], [8.2, ty, -3.4]]),
      wires.add([[9.2, ty, -4.1], [9.2, ty, -3.55], [8.2, ty, -3.55], [8.2, ty, -3.4]]),
      wires.add([[12.0, ty, -4.1], [12.0, ty, -3.4]]),
      wires.add([[14.2, ty, -4.1], [14.2, ty, -3.4]]),
    ];
    root.add(wires.finish(UB));

    // --- inset tray, its tiles and the equality ring
    const tray = new THREE.Group();
    tray.position.set(11, 0, -3.2);
    const trayFloor = mesh(UB, mFloor);
    setBox(trayFloor, 0, 0.05, 0, 9.0, 0.1, 4.5);
    const trayRim = inst(UB, mTi, 4);
    setInst(trayRim, 0, 0, 0.07, -2.19, 9.0, 0.14, 0.12);
    setInst(trayRim, 1, 0, 0.07, 2.19, 9.0, 0.14, 0.12);
    setInst(trayRim, 2, -4.44, 0.07, 0, 0.12, 0.14, 4.26);
    setInst(trayRim, 3, 4.44, 0.07, 0, 0.12, 0.14, 4.26);
    trayRim.instanceMatrix.needsUpdate = true;
    tray.add(trayFloor, trayRim);
    root.add(tray);
    o.tray = tray;
    o.tileXZ = [[7.2, -4.5], [9.2, -4.5], [8.2, -3.0], [12.0, -4.5], [14.2, -4.5], [12.0, -3.0], [14.2, -3.0]];
    o.tiles = inst(UB, mCeramic, 7);
    o.tileBars = inst(UB, mInk, 7);
    root.add(o.tiles, o.tileBars);
    o.ring = mesh(TORUS, mGold);
    root.add(o.ring);

    // --- input commitment: domain cell, collars, rulers
    o.shk = mesh(UB, mGold);
    o.shkCopy = mesh(UB, mGold);
    root.add(o.shk, o.shkCopy);
    o.channels = COLLAR_X.map(() => { const m = mesh(UB, mTi); root.add(m); return m; });
    o.cubes = inst(UB, mInput, 24);
    o.chCopy = inst(UB, mTi, 3);
    o.cubesCopy = inst(UB, mInput, 24);
    root.add(o.cubes, o.chCopy, o.cubesCopy);
    o.strips = inst(UB, mTi, 3);
    o.ticks = inst(UB, mLine, 21);
    root.add(o.strips, o.ticks);

    // --- key rack and cards
    const rack = new THREE.Group();
    rack.position.set(4, 4.2, -5.4);
    const rail = mesh(UB, mTi);
    setBox(rail, 0, 0, 0, 14, 0.15, 0.15);
    const posts = inst(UB, mTi, 2);
    setInst(posts, 0, -6.8, -2.1, 0, 0.25, 4.2, 0.25);
    setInst(posts, 1, 6.8, -2.1, 0, 0.25, 4.2, 0.25);
    posts.instanceMatrix.needsUpdate = true;
    rack.add(rail, posts);
    o.cards = CARD_X.map((x) => {
      const m = mesh(UB, mGold);
      setBox(m, x - 4, -0.65, 0, 1.6, 1.0, 0.08);
      rack.add(m);
      return m;
    });
    const notches = inst(UB, mInk, 6);
    {
      let k = 0;
      [3, 2, 1].forEach((n, ci) => {
        for (let j = 0; j < n; j++) setInst(notches, k++, CARD_X[ci] - 4 + (j - (n - 1) / 2) * 0.4, -1.1, 0.05, 0.16, 0.12, 0.1);
      });
      notches.instanceMatrix.needsUpdate = true;
    }
    rack.add(notches);
    root.add(rack);
    o.rack = rack;

    // --- verification sweeps
    o.sweeps = [];
    for (let i = 0; i < 7; i++) { const m = mesh(UB, mSweep); m.visible = false; root.add(m); o.sweeps.push(m); }

    // --- anchors for labels
    const mk = (name, parent, x, y, z) => {
      const a = new THREE.Object3D();
      a.position.set(x, y, z);
      parent.add(a);
      ctx.labels.anchor(name, a);
      return a;
    };
    mk('sheetLabel', sheet, -1.2, 0.1, -2.7);
    mk('inpLabel', root, INP[0], 0.6, INP[2]);
    mk('liftLabel', sheet, 1.7, 0.1, barZ(0));
    mk('condLabel', sheet, 1.6, 0.1, barZ(7));
    mk('pressLabel', root, PRESS[0], 1.7, PRESS[1]);
    mk('romLabel', rom, 0, 1.7, 0);
    mk('tablesA', rom, 2.2, 0.6, 0);
    mk('tablesB', rom, 2.2, -0.6, 0);
    mk('hashPLabel', root, HASH_P[0], 1.0, HASH_P[1]);
    ctx.labels.anchor('tokPLabel', o.tokP.userData.anchor);
    mk('cellsLabel', root, -1, 0.3, 2.5);
    mk('hashVLabel', root, HASH_V[0], 1.0, HASH_V[1]);
    ctx.labels.anchor('tokVLabel', o.tokV.userData.anchor);
    mk('eqLabel', root, 8.5, 0.9, -2.0);
    mk('shkLabel', root, SHK[0], 0.4, SHK[2]);
    COLLAR_X.forEach((x, i) => mk(`collar${i}`, root, x, 0.3, LANE_Z));
    ctx.labels.anchor('tokILabel', o.tokI.userData.anchor);
    CARD_X.forEach((x, i) => mk(`card${i}`, rack, x - 4, -0.15, 0));

    // --- camera
    const s = ctx.camera.shots;
    ctx.camera.track([
      { t: 0, pose: s.establish([-10, 1, 0], 30), dur: 2, cut: true },
      { t: B[1], pose: s.establish([-6, 1, -0.5], 20), dur: 1.5 },
      { t: B[2], pose: s.establish([2, 1, -1.5], 16), dur: 1.5 },
      { t: B[3], pose: s.establish([6, 1, 0.5], 22), dur: 1.5 },
      { t: B[4], pose: s.establish([8.5, 1, 3], 14), dur: 1.5 },
      { t: B[5], pose: s.establish([2, 1.5, 0], 36), dur: 1.5 },
      { t: B[5] + 13, pose: s.establish([10, 1, 0], 30), dur: 1.5 },
    ]);
  },

  seek(tt) {
    t = tt;
    const O = o;
    const UBs = 1;
    void UBs;

    // ---------- sheet and input cube (beat 0)
    const sx = lerp(-15.5, SHEET[0], ph(0, 2.0, 3.2));
    O.sheet.position.set(sx, SHEET[1], SHEET[2]);
    useLit(O.liftBars, O.mProg, O.mProgLit, 0.5 * pulse(0, 5.0, 8.0));
    // each lit bar set uses the shared lit material, so the two pulses never overlap in time
    useLit(O.condBars, O.mProg, O.mProgLit, 0.5 * pulse(0, 9.0, 12.0));
    {
      const k = ph(0, 2.0, 3.2);
      const gone = 1 - ph(3, 3.0, 3.5);
      const mv = ph(3, 2.0, 3.0);
      O.inp.position.set(lerp(INP[0], -1, mv), INP[1], lerp(INP[2], 2.5, mv));
      const s = k * gone;
      O.inp.scale.setScalar(Math.max(0.6 * s, 0.001));
      O.inp.visible = s > 0.002;
    }

    // ---------- press (beats 0 to 1)
    const lower = ph(1, 3.4, 3.8) - ph(1, 3.8, 4.2);
    const py = 1.3 - 0.25 * lower;
    setBox(O.platen, PRESS[0], py, PRESS[1], 3.0, 0.8, 5.6);
    setInst(O.rim, 0, PRESS[0], 0.54, PRESS[1] + 2.78, 3.1, 0.08, 0.1);
    setInst(O.rim, 1, PRESS[0], py - 0.4, PRESS[1] + 2.78, 3.1, 0.08, 0.1);
    setInst(O.rim, 2, PRESS[0], py + 0.4, PRESS[1] + 2.78, 3.1, 0.08, 0.1);
    O.rim.instanceMatrix.needsUpdate = true;
    {
      // copy of the sheet slides into the slot, then is pressed away
      const cx = lerp(SHEET[0], PRESS[0], ph(1, 2.0, 3.4));
      const cy = lerp(0.1, 0.6, ph(1, 2.0, 2.8));
      const cs = 0.6 * (within(1, 2.0, 22) ? 1 - ph(1, 4.2, 4.6) : 0);
      O.copy.position.set(cx, cy, PRESS[1]);
      O.copy.scale.setScalar(Math.max(cs, 0.001));
      O.copy.visible = cs > 0.002;
    }

    // ---------- ROM rack (beats 1 to 5)
    const frameK = ph(1, 4.4, 4.8);
    O.frame.scale.setScalar(Math.max(frameK, 0.001));
    O.frame.visible = frameK > 0.002;
    const shelfPos = (i) => {
      const s0 = 4.5 + 0.5 * i;
      const e = ph(1, s0, s0 + 0.5);
      const sc = ramp(t, B[1] + s0, B[1] + s0 + 0.15);
      _v[0] = lerp(-3.4, 0, e);
      _v[1] = lerp(-0.6, SHELF_Y[i], e);
      _v[2] = lerp(1.0, 0, e);
      return sc;
    };
    for (let i = 0; i < 5; i++) {
      const sc = shelfPos(i);
      const sh = O.shelves[i];
      sh.position.set(_v[0], _v[1], _v[2]);
      sh.scale.set(2.6 * sc, 0.08 * sc, 1.2 * sc);
      sh.visible = sc > 0.002;
      for (let j = 0; j < 6; j++) setInst(O.cellsRom, i * 6 + j, _v[0] + (-1.0 + 0.4 * j) * sc, _v[1] + 0.165 * sc, _v[2], 0.25 * sc, 0.25 * sc, 0.25 * sc);
    }
    O.cellsRom.instanceMatrix.needsUpdate = true;
    {
      // handoff slide of the rack at the end of beat 5
      const hs = ph(5, 15.0, 17.2);
      const zs = ph(5, 15.0, 15.9);
      O.rom.position.set(lerp(ROM0[0], 20, hs), lerp(ROM0[1], 1.5, hs), lerp(ROM0[2], 0, zs));
      O.rom.visible = frameK > 0.002;
    }
    // verification sweep across the rack (beat 1, 8.0)
    {
      const k = rp(1, 8.0, 8.4);
      const sw = O.sweeps[0];
      sw.visible = k > 0 && k < 1;
      setBox(sw, ROM0[0] + lerp(-1.5, 1.5, k), ROM0[1], ROM0[2] + 0.85, 0.08, 2.6, 0.05);
      useLit(O.frame, O.mTi, O.mTiLit, within(1, 8.4, 8.7) * 0.8);
    }

    // ---------- hash block P (beat 2)
    const romW = O.rom.position;
    const tlP = t - B[2] - 2.0;
    hashBlock(O.hashP, ph(2, 0.3, 0.7), glowOf(tlP));
    {
      // five copies of the shelves, scaled to cells, fly into the block
      for (let i = 0; i < 5; i++) {
        const A = [ROM0[0], ROM0[1] + SHELF_Y[i], ROM0[2]];
        const F = [1.8 - 0.6 * i, 0.5, HASH_P[1]];
        const kk = flow(tlP, 5, i, A, F, SLOT_P);
        const vis = tlP > 0 ? 1 : 0;
        const sc = 1 - kk[1];
        setInst(O.romCopies, i, _v[0], _v[1], _v[2],
          vis * lerp(2.6, 0.4, kk[0]) * sc, vis * lerp(0.08, 0.4, kk[0]) * sc, vis * lerp(1.2, 0.4, kk[0]) * sc);
      }
      O.romCopies.instanceMatrix.needsUpdate = true;
    }
    {
      const k = ph(2, 4.2, 5.0) * after(2, 4.2);
      place(O.tokP, lerp(HASH_P[0] + 1.2, TOK_P[0], k), lerp(0.5, TOK_P[1], k), TOK_P[2], lerp(0.2, 1, k) * after(2, 4.2));
    }
    // connector rack to token, retracted when the rack leaves
    wireSet(O.wTokP, ph(2, 6.0, 6.8) * (1 - ph(5, 15.0, 15.5)));
    // copy of the token travels to the carrier
    {
      const k = ph(2, 8.0, 10.0);
      const on = within(2, 8.0, 10.0);
      const sc = (lerp(1, 0.3, k)) * on * (1 - rp(2, 9.8, 10.0));
      place(O.travP, lerp(TOK_P[0], CARRIER_PT[0], k), lerp(TOK_P[1], CARRIER_PT[1], k) + 1.2 * Math.sin(Math.PI * k), lerp(TOK_P[2], CARRIER_PT[2], k), sc);
    }

    // ---------- value digest (beat 3)
    {
      const tlV = t - B[3] - 3.5;
      hashBlock(O.hashV, ph(3, 0.5, 0.9), glowOf(tlV));
      const ka = ph(3, 3.0, 3.5);
      for (let i = 0; i < 4; i++) {
        const A = [-1.75 + 0.5 * i, 0.2, 2.5];
        const F = [1.7 - 0.6 * i, 0.5, 2.5];
        const kk = flow(tlV, 4, i, A, F, SLOT_V);
        const sc = ka * (1 - kk[1]);
        setInst(O.cells, i, _v[0], _v[1], _v[2], 0.4 * sc, 0.4 * sc, 0.4 * sc);
      }
      O.cells.instanceMatrix.needsUpdate = true;
      const k = ph(3, 5.7, 6.5) * after(3, 5.7);
      place(O.tokV, lerp(HASH_V[0] + 1.2, TOK_V[0], k), lerp(0.5, TOK_V[1], k), TOK_V[2], lerp(0.2, 1, k) * after(3, 5.7));
    }
    // inset with the shared and the separate structure (beat 3 only), then rom2 (beat 5)
    {
      const rise = ph(3, 7.0, 7.6) * (t < B[4] ? 1 : 0) + ph(5, 2.6, 3.2);
      O.tray.scale.set(1, Math.max(rise, 0.001), 1);
      O.tray.visible = rise > 0.002;
      const c3 = ph(3, 7.4, 8.0) * (t < B[4] ? 1 : 0);
      for (let i = 0; i < 7; i++) {
        const p = O.tileXZ[i];
        setInst(O.tiles, i, p[0], 0.16, p[1], 0.8 * c3, 0.12 * c3, 0.8 * c3);
        setInst(O.tileBars, i, p[0], 0.23, p[1], 0.5 * c3, 0.02 * c3, 0.1 * c3);
      }
      O.tiles.instanceMatrix.needsUpdate = true;
      O.tileBars.instanceMatrix.needsUpdate = true;
      for (const w of O.wIn) wireSet(w, c3);
      const mv = ph(3, 9.0, 10.0);
      const eq = t >= B[3] + 10.0 && t < B[4] ? 1 : 0;
      place(O.valL, 8.5, 0.5, -2.0, c3);
      place(O.valR, lerp(13.5, 8.5, mv), 0.5, -2.0, c3 * (1 - eq));
      O.ring.position.set(8.5, 0.5, -2.0);
      O.ring.visible = eq > 0;
      O.ring.material = within(3, 10.0, 10.3) ? O.mGoldLit : O.mGold;
      if (within(3, 10.0, 10.3)) O.mGoldLit.emissiveIntensity = 0.5;
      // rom2 and tokP2 on the inset (beat 5)
      const r2 = ph(5, 2.8, 3.2);
      place(O.rom2, 9.0, 0.1 + 1.3 * 0.6 * r2, -3.2, 0.6 * r2);
      O.rom2.position.y = 0.1 + 1.3 * 0.6 * r2;
      place(O.tokP2, 12.5, 0.5, -3.2, r2);
    }

    // ---------- input commitment (beat 4)
    {
      const tlI = t - B[4] - 6.0;
      hashBlock(O.hashI, ph(4, 1.0, 1.4), glowOf(tlI));
      const ap = ph(4, 2.0, 2.6);
      const on4 = t >= B[4] ? 1 : 0;
      place(O.shk, SHK[0], SHK[1], SHK[2], ap * on4);
      for (let i = 0; i < 3; i++) {
        const cx = COLLAR_X[i];
        const s = ap * on4;
        setBox(O.channels[i], cx, 0.06 * s, LANE_Z, 1.9 * s, 0.12 * s, 0.3 * s);
        O.channels[i].visible = s > 0.002;
        for (let j = 0; j < 8; j++) setInst(O.cubes, i * 8 + j, cx + (j - 3.5) * 0.22 * s, 0.22 * s, LANE_Z, 0.2 * s, 0.2 * s, 0.2 * s);
        // ruler under the collar
        const rk = ph(4, 2.6 + 0.3 * i, 3.0 + 0.3 * i) * on4;
        const rz = LANE_Z + 0.55;
        setInst(O.strips, i, cx, 0.05 * rk, rz, 1.9 * rk, 0.1 * rk, 0.3 * rk);
        const tk = [-0.85, -0.55, -0.25, 0.5, 0.95];
        for (let q = 0; q < 3; q++) setInst(O.ticks, i * 7 + q, cx + tk[q] * rk, 0.11 * rk, rz, 0.03 * rk, 0.02 * rk, 0.16 * rk);
        setInst(O.ticks, i * 7 + 3, cx + 0.5 * rk, 0.11 * rk, rz, 0.03 * rk, 0.02 * rk, 0.16 * rk);
        setInst(O.ticks, i * 7 + 4, cx + 0.12 * rk, 0.11 * rk, rz, 0.03 * rk, 0.02 * rk, 0.26 * rk, _qBreak);
        setInst(O.ticks, i * 7 + 5, cx + 0.2 * rk, 0.11 * rk, rz, 0.03 * rk, 0.02 * rk, 0.26 * rk, _qBreak);
        setInst(O.ticks, i * 7 + 6, cx + 0.95 * rk, 0.11 * rk, rz, 0.04 * rk, 0.03 * rk, 0.28 * rk);
        // sweep over the collar
        const st = 3.0 + 0.8 * i;
        const k = rp(4, st, st + 0.4);
        const sw = O.sweeps[1 + i];
        sw.visible = k > 0 && k < 1;
        setBox(sw, cx + lerp(-0.95, 0.95, k), 0.2, LANE_Z, 0.08, 0.5, 0.5);
        useLit(O.channels[i], O.mTi, O.mTiLit, within(4, st + 0.4, st + 0.7) * 0.8);
      }
      O.cubes.instanceMatrix.needsUpdate = true;
      O.strips.instanceMatrix.needsUpdate = true;
      O.ticks.instanceMatrix.needsUpdate = true;
      O.cubes.visible = O.strips.visible = O.ticks.visible = on4 > 0;

      // copies fly into the hash block in a raised file
      const FY = 2.0;
      const item = (i, A, sA, sF) => {
        const F = [8.0 - 0.75 * i, FY, 2.5];
        const kk = flow(tlI, 5, i, A, F, SLOT_I);
        const vis = tlI > 0 ? 1 : 0;
        return [kk, vis, lerp(sA, sF, kk[0]) * (1 - kk[1]) * vis];
      };
      {
        const [, , sc] = item(0, SHK, 1, 0.8);
        place(O.shkCopy, _v[0], _v[1], _v[2], sc);
        O.shkCopy.scale.setScalar(Math.max(0.4 * sc, 0.001));
      }
      {
        const [, , sc] = item(1, TOK_V, 1, 0.45);
        const vx = _v[0], vy = _v[1], vz = _v[2];
        const useV = t < B[4] + 10.0;
        O.travI.userData.setContent(useV ? 'val:input' : 'input:main');
        // the second user of the same token is the input_d copy, below
        const sV = useV ? sc : 0;
        const kc = ph(4, 10.0, 12.0);
        const onC = within(4, 10.0, 12.0) * (1 - rp(4, 11.8, 12.0));
        if (useV) place(O.travI, vx, vy, vz, sV);
        else place(O.travI, lerp(TOK_I[0], CARRIER_PT[0], kc), lerp(TOK_I[1], CARRIER_PT[1], kc) + 1.2 * Math.sin(Math.PI * kc), lerp(TOK_I[2], CARRIER_PT[2], kc), lerp(1, 0.3, kc) * onC);
      }
      for (let i = 0; i < 3; i++) {
        const A = [COLLAR_X[i], 0, LANE_Z];
        const F2 = 2 + i;
        const F = [8.0 - 0.75 * F2, FY, 2.5];
        const kk = flow(tlI, 5, F2, A, F, SLOT_I);
        const vis = tlI > 0 ? 1 : 0;
        const sc = lerp(1, 0.3, kk[0]) * (1 - kk[1]) * vis;
        setInst(O.chCopy, i, _v[0], _v[1] + 0.06 * sc, _v[2], 1.9 * sc, 0.12 * sc, 0.3 * sc);
        for (let j = 0; j < 8; j++) setInst(O.cubesCopy, i * 8 + j, _v[0] + (j - 3.5) * 0.22 * sc, _v[1] + 0.22 * sc, _v[2], 0.2 * sc, 0.2 * sc, 0.2 * sc);
      }
      O.chCopy.instanceMatrix.needsUpdate = true;
      O.cubesCopy.instanceMatrix.needsUpdate = true;
      const k = ph(4, 8.2, 9.0) * after(4, 8.2);
      place(O.tokI, lerp(HASH_I[0] + 1.2, TOK_I[0], k), lerp(0.5, TOK_I[1], k), TOK_I[2], lerp(0.2, 1, k) * after(4, 8.2));
    }

    // ---------- keys (beat 5)
    {
      const rk = ph(5, 2.0, 2.4);
      O.rack.scale.setScalar(Math.max(rk, 0.001));
      O.rack.visible = rk > 0.002;
      for (let i = 0; i < 3; i++) {
        const st = 3.5 + 0.7 * i;
        const k = rp(5, st, st + 0.4);
        const sw = O.sweeps[4 + i];
        sw.visible = k > 0 && k < 1;
        setBox(sw, CARD_X[i] + lerp(-0.8, 0.8, k), 3.55, -5.32, 0.08, 1.0, 0.12);
        useLit(O.cards[i], O.mGold, O.mGoldLit, within(5, st + 0.4, st + 0.7) * 0.8);
      }
      for (let i = 0; i < 3; i++) wireSet(O.wDrop[i], ph(5, 6.0, 6.4));
      wireSet(O.wMain, ph(5, 6.3, 6.8));
      wireSet(O.wBranchRom, ph(5, 6.6, 7.0) * (1 - ph(5, 15.0, 15.5)));
      wireSet(O.wBranchRom2, ph(5, 6.6, 7.0));
    }
    O.wires.commit();
    O.sweeps[0].parent.updateMatrixWorld();
    void romW;

    // ---------- carrier
    ctx.carrier.show('prog_d', ph(2, 8.0, 10.0), 'prog:main');
    ctx.carrier.show('input_d', ph(4, 10.0, 12.0), 'input:main');
    ctx.carrier.show('outcome_d', 0);
    ctx.carrier.show('n_steps', 0);
  },

  dispose() {
    if (o) {
      o.tokP.userData.dispose?.();
      for (const k of ['tokP', 'tokV', 'tokI', 'tokP2', 'travP', 'travI', 'valL', 'valR']) {
        o[k].traverse((n) => {
          if (n.geometry) n.geometry.dispose();
          if (n.material) (Array.isArray(n.material) ? n.material : [n.material]).forEach((m) => m.dispose());
        });
      }
    }
    for (const x of owned) x.dispose();
    owned = [];
    o = null;
    ctx = null;
  },
};

// ---- helpers that need the module state -------------------------------------------------------
function wireSet(poly, g) { o.wires.set(poly, g); }

function hashBlock(h, appear, glow) {
  const s = Math.max(appear, 0.001);
  const vis = appear > 0.002;
  setBox(h.block, h.x, 0.5 * s, h.z, 2.4 * s, 1.0 * s, 1.2 * s);
  h.block.visible = vis;
  h.slots.visible = vis;
  setInst(h.slots, 0, h.x - 1.2 * s, 0.5 * s, h.z, 0.1 * s, 0.4 * s, 0.6 * s);
  setInst(h.slots, 1, h.x + 1.2 * s, 0.5 * s, h.z, 0.1 * s, 0.4 * s, 0.6 * s);
  h.slots.instanceMatrix.needsUpdate = true;
  useLit(h.block, o.mGold, o.mGoldLit, glow);
}

void FAMILY;
