// Scene 2, "The run": the reference machine fires one rule per step and prints one frame per step.
// Every property is a pure function of the scene time T: a beat's motion is phase(T, start + a, start + b),
// so the end pose of an earlier beat is simply the settled value of the same expression.
import * as THREE from 'three/webgpu';
import { PALETTE, FAMILY, materials, makeDigestToken, lerp, phase, ramp, clamp01 } from '../theme.js';
import { pow2 } from '../labels.js';
import { BEATS } from './scene2/text.js';

// ---- timing --------------------------------------------------------------------------------------
const S = [0, 23, 47, 71, 95, 119, 142, 165, 186, 209];
let T = 0;
const W = (i, a, b) => phase(T, S[i] + a, S[i] + b);
const R = (i, a, b) => ramp(T, S[i] + a, S[i] + b);
const IN = (i, a, b) => T >= S[i] + a && T < S[i] + b;
const PI = Math.PI;

// ---- scratch objects (no allocation inside seek) ---------------------------------------------------------
const Q = new THREE.Quaternion();
const V = new THREE.Vector3();
const SC = new THREE.Vector3();
const M4 = new THREE.Matrix4();
const UP = new THREE.Vector3(0, 1, 0);
const MIX = new THREE.Color();
const TX = { ox: 0, oy: 0, oz: 0, ry: 0, s: 1, c: 1, sn: 0 };
const WP = new THREE.Vector3();

const col = (hex) => new THREE.Color(hex);
let C = null;      // colours
let pools = null;  // matte, metal, lit
let ctx = null;
let geo = null;
let toks = null;
let anchors = null;
let ownedMats = [];

const mix = (a, b, k) => MIX.copy(a).lerp(b, clamp01(k));

class Pool {
  constructor(geometry, material, cap) {
    this.cap = cap;
    this.n = 0;
    this.mesh = new THREE.InstancedMesh(geometry, material, cap);
    this.mesh.frustumCulled = false;
    this.mesh.count = 0;
    this.mesh.setColorAt(0, new THREE.Color());
  }
  reset() { this.n = 0; }
  add(x, y, z, w, h, d, color, ry) {
    if (this.n >= this.cap) return;
    Q.setFromAxisAngle(UP, ry);
    V.set(x, y, z);
    SC.set(w, h, d);
    M4.compose(V, Q, SC);
    this.mesh.setMatrixAt(this.n, M4);
    this.mesh.setColorAt(this.n, color);
    this.n++;
  }
  done() {
    this.mesh.count = this.n;
    this.mesh.instanceMatrix.needsUpdate = true;
    if (this.mesh.instanceColor) this.mesh.instanceColor.needsUpdate = true;
  }
}

function setT(ox, oy, oz, ry, s) {
  TX.ox = ox; TX.oy = oy; TX.oz = oz; TX.ry = ry; TX.s = s;
  TX.c = Math.cos(ry); TX.sn = Math.sin(ry);
}
// Local to world through the current transform.
function tw(x, y, z, out) {
  return out.set(TX.ox + (x * TX.c + z * TX.sn) * TX.s, TX.oy + y * TX.s, TX.oz + (-x * TX.sn + z * TX.c) * TX.s);
}
function box(pool, x, y, z, w, h, d, color, ry = 0) {
  const s = TX.s;
  if (s < 0.002) return;
  pool.add(
    TX.ox + (x * TX.c + z * TX.sn) * s, TX.oy + y * s, TX.oz + (-x * TX.sn + z * TX.c) * s,
    w * s, h * s, d * s, color, TX.ry + ry,
  );
}
// Rectangular frame of four thin bars around a footprint (also the gold outline ring).
function ring(pool, x, y, z, w, d, th, h, color) {
  box(pool, x, y, z - d / 2, w + th, h, th, color);
  box(pool, x, y, z + d / 2, w + th, h, th, color);
  box(pool, x - w / 2, y, z, th, h, d, color);
  box(pool, x + w / 2, y, z, th, h, d, color);
}
// Thin line along an axis-aligned polyline [[x,z],...], drawn up to fraction f of its length.
function poly(pool, pts, f, y, color) {
  if (f <= 0.001) return;
  let total = 0;
  for (let i = 1; i < pts.length; i++) total += Math.abs(pts[i][0] - pts[i - 1][0]) + Math.abs(pts[i][1] - pts[i - 1][1]);
  let left = f * total;
  for (let i = 1; i < pts.length && left > 0; i++) {
    const [x0, z0] = pts[i - 1];
    const [x1, z1] = pts[i];
    const len = Math.abs(x1 - x0) + Math.abs(z1 - z0);
    const u = Math.min(left, len);
    left -= u;
    if (len <= 0) continue;
    const ex = x0 + ((x1 - x0) / len) * u;
    const ez = z0 + ((z1 - z0) / len) * u;
    const w = Math.abs(ex - x0) + 0.06;
    const d = Math.abs(ez - z0) + 0.06;
    box(pool, (x0 + ex) / 2, y, (z0 + ez) / 2, w, 0.04, d, color);
  }
}
// A verification sweep: a bright bar crosses in 0.4 s, then the outline glows for 0.3 s.
function sweepBar(t0, x0, x1, y, z, len) {
  const k = (T - t0) / 0.4;
  if (k > 0 && k < 1) box(pools.lit, lerp(x0, x1, k), y, z, 0.08, 0.04, len, C.goldB);
}
const glowing = (t0) => T >= t0 + 0.4 && T < t0 + 0.7;

// ---- fixed layout ----------------------------------------------------------------------------------------
const BAYS = [
  { cx: -9.8, cz: -1.5 }, // control
  { cx: -6, cz: -1.5 },   // environment
  { cx: -9.8, cz: 1.5 },  // frame stack
  { cx: -6, cz: 1.5 },    // task stack
  { cx: -2.2, cz: 1.5 },  // memory
  { cx: -2.2, cz: -1.5 }, // fuel
];
const bayS = (k) => phase(T, S[1] + 3.5 + 0.4 * k, S[1] + 3.9 + 0.4 * k);

const KEY_COLS = 12;
const KEY_ROWS = 10;
const HOT = { c: 6, r: 3 };
const keyX = (c) => 3.4 + (c - 5.5) * 0.6;
const keyZ = (r) => -2.5 + (r - 4.5) * 0.5;
const HOT_X = keyX(HOT.c);
const HOT_Z = keyZ(HOT.r);

const BAR_LEN = [2.4, 1.6, 0, 3.0, 2.2, 2.8, 1.2, 3.2, 2.0, 2.6, 1.8, 2.4];
const barZ = (k) => 4.2 + 0.5 * (-2.3 + 0.42 * k);
const barRight = (k) => -14.2 + 0.5 * (-1.7 + BAR_LEN[k]);
const K_ADD = 5;
const K_CALL = 7;
const K_BOOL = 9;

const P_NEXT = [[HOT_X, HOT_Z], [-8.55, HOT_Z], [-8.55, -1.3], [-8.7, -1.3]];
const P_CALL = [[barRight(K_CALL), barZ(K_CALL)], [-9.8, barZ(K_CALL)], [-9.8, 2.8]];
const P_ADD = [[barRight(K_ADD), barZ(K_ADD)], [-10.6, barZ(K_ADD)], [-10.6, -0.35]];
const P_BOOL = [[barRight(K_BOOL), barZ(K_BOOL)], [-9.2, barZ(K_BOOL)], [-9.2, -0.35]];
const P_ENV = [[-5.5, -1.8], [-4.1, -1.8], [-4.1, 1.5], [-3.75, 1.5]];
const P_TAPE = [[HOT_X, HOT_Z], [HOT_X, 2.6], [2.5, 2.6], [2.5, 3.45]];
const LINE_Y = 0.24;

// ---- the tape ----------------------------------------------------------------------------------------------
const LANE_Z = 3.6;
const PRINTS = [
  { d: S[5] + 2.0, a0: S[5] + 2.4, a1: S[5] + 3.2 },
  { d: S[6] + 2.0, a0: S[6] + 2.4, a1: S[6] + 3.2 },
  { d: S[6] + 3.4, a0: S[6] + 3.8, a1: S[6] + 4.6 },
  { d: S[6] + 4.8, a0: S[6] + 5.2, a1: S[6] + 6.0 },
  { d: S[9] + 2.0, a0: S[9] + 2.4, a1: S[9] + 3.2 },
];
const FRAMES = [
  { id: 'A', x0: 3.5, from: 0, kept: true, loc: -2.15 },
  { id: 'B', x0: 2.5, from: 0, kept: true, loc: -1.15 },
  { id: 'C', x0: 1.5, from: 0, own: 0, kept: false },
  { id: 'D', x0: 1.5, from: 1, own: 1, kept: false },
  { id: 'E', x0: 1.5, from: 2, own: 2, kept: false },
  { id: 'F', x0: 1.5, from: 3, own: 3, kept: true, loc: 0.1 },
  { id: 'G', x0: 1.5, from: 4, own: 4, kept: true, loc: 1.1 },
];
function frameX(f) {
  let x = f.x0;
  for (let k = f.from; k < PRINTS.length; k++) x += phase(T, PRINTS[k].a0, PRINTS[k].a1);
  return x;
}
function headDip() {
  let dy = 0;
  for (const p of PRINTS) {
    const r = ramp(T, p.d, p.d + 0.4);
    if (r > 0 && r < 1) dy = -0.2 * Math.sin(PI * r);
  }
  return dy;
}
const GROUP_BASE_X = 9;
const NPLATE_LOC = 2.25;
const BREAK_LOC = -0.55;

const FP = new THREE.Vector3();   // frame anchors (world)
const FR_STATE = {};              // id -> { x, z, ry, s, y }

// One witness frame in its local axes: the state before on the +x side, the state after on the -x side,
// so that time runs toward -x along the tape. Turned by half a turn it reads from -x to +x.
function drawFrame(f, x, y, z, ry, s, o) {
  setT(x, y, z, ry, s);
  const { matte } = pools;
  box(matte, 0, 0.06, 0, 0.9, 0.06, 1.2, C.ceramic);
  box(matte, 0, 0.13, 0, 0.3, 0.08, 0.5, o.rule);
  const bo = o.beforeOff || 0;
  const bs = o.beforeS === undefined ? 1 : o.beforeS;
  for (let i = -1; i <= 1; i++) {
    if (bs > 0.01) box(matte, 0.3 + bo, 0.18, i * 0.3, 0.18 * bs, 0.18 * bs, 0.18 * bs, C.inkSoft);
    if (!o.cube) box(matte, -0.3, 0.18, i * 0.3, 0.18, 0.18, 0.18, C.inkSoft);
  }
  if (o.cube) box(matte, -0.3, 0.26, 0, 0.4 * o.cube, 0.4 * o.cube, 0.4 * o.cube, C.outcome);
  const st = FR_STATE[f.id] || (FR_STATE[f.id] = {});
  st.x = x; st.y = y; st.z = z; st.ry = ry; st.s = s;
  setT(0, 0, 0, 0, 1);
}
function frameWorld(id, lx, ly, lz, out) {
  const st = FR_STATE[id];
  setT(st.x, st.y, st.z, st.ry, st.s);
  tw(lx, ly, lz, out);
  setT(0, 0, 0, 0, 1);
  return out;
}

// ---- scene body ----------------------------------------------------------------------------------------------
const LABEL_NAMES = [
  'housing', 'socket', 'refPlate', 'tapePath',
  'bCtrl', 'bEnv', 'bFrame', 'bTask', 'bMem', 'bFuel',
  'rules', 'curState', 'romRow', 'hotKey', 'nextState',
  'callExpr', 'pushed', 'addRes', 'boolRes', 'savedIn',
  'gInit', 'gCharge', 'gRemain', 'gWork', 'gLimits',
  'ruleId', 'stBefore', 'stAfter', 'eqCells',
  'counter', 'nPlate', 'limit', 'rulerA',
  'ovf', 'rejTag', 'rejPay',
  'chargeB', 'gaugeB', 'exhTag', 'exhPay',
  'haltTape', 'haltTag', 'resCube',
];

const lab = (text, anchor, kind, dy, extra = {}) => ({ text, anchor, kind, dy, ...extra });
const code = (s) => `<code>${s}</code>`;

const LABELS = [
  [
    lab(code('mori simulate'), 'housing', 'neutral', 0.7),
    lab('Program image', 'socket', 'program', 0.7),
    lab(code('sherlock-k-a/1'), 'refPlate', 'neutral', 0.8),
    lab('Witness stream', 'tapePath', 'neutral', 1.2),
  ],
  [
    lab('Control', 'bCtrl', 'neutral', 0.7),
    lab('Environment', 'bEnv', 'neutral', 0.9),
    lab('Frame stack', 'bFrame', 'neutral', 0.9),
    lab('Task stack', 'bTask', 'neutral', 0.9),
    lab('Memory', 'bMem', 'neutral', 0.9),
    lab('Fuel counters', 'bFuel', 'neutral', 0.9),
  ],
  [
    lab('120 rules', 'rules', 'neutral', 0.6),
    lab('Current state', 'curState', 'neutral', 1.6),
    lab('Instruction', 'romRow', 'program', 1.5),
    lab('Selected rule', 'hotKey', 'step', 1.0),
    lab('Next state', 'nextState', 'step', 0.5, { from: 5.0 }),
  ],
  [
    lab('Named call', 'callExpr', 'step', 0.6),
    lab('Saved continuation', 'pushed', 'state', 0.9, { from: 3.2 }),
    lab('Raised value', 'addRes', 'neutral', 0.7, { from: 5.5 }),
    lab('Branch condition', 'boolRes', 'neutral', 1.2, { from: 5.5 }),
    lab('Saved input', 'savedIn', 'input', 1.0, { from: 6.0 }),
  ],
  [
    lab('Initial budget', 'gInit', 'neutral', 1.5),
    lab('Required charge', 'gCharge', 'step', 0.9, { from: 2.6 }),
    lab('Remaining fuel', 'gRemain', 'neutral', 0.7),
    lab('Consumed work', 'gWork', 'neutral', 1.5),
    lab('Resource limits', 'gLimits', 'neutral', 0.9),
  ],
  [
    lab('Witness stream', 'haltTape', 'neutral', 0.7),
    lab('Rule id', 'ruleId', 'step', 1.1),
    lab('State before', 'stBefore', 'neutral', 1.1),
    lab('State after', 'stAfter', 'neutral', 0.4),
    lab('Equal boundary', 'eqCells', 'state', 1.8, { from: 4.8 }),
  ],
  [
    lab('64-bit counter', 'counter', 'neutral', 0.8),
    lab(code('n_steps'), 'nPlate', 'state', 0.9),
    lab(`${pow2(64)} - 1 steps`, 'limit', 'neutral', 0.6, { from: 7.0 }),
    lab('Counter range', 'rulerA', 'neutral', 1.9, { from: 7.0 }),
  ],
  [
    lab('Arithmetic overflow', 'ovf', 'error', 0.8),
    lab('Rejected', 'rejTag', 'error', 0.8, { from: 3.4 }),
    lab('Code 10', 'rejPay', 'error', 1.7, { from: 3.4 }),
  ],
  [
    lab('Required charge', 'chargeB', 'step', 0.9),
    lab('Remaining fuel', 'gaugeB', 'neutral', 0.9),
    lab('Exhausted', 'exhTag', 'error', 0.8, { from: 3.4 }),
    lab('Fuel', 'exhPay', 'error', 1.7, { from: 3.4 }),
  ],
  [
    lab('Witness stream', 'haltTape', 'neutral', 1.2),
    lab('Halted', 'haltTag', 'outcome', 2.7),
    lab('Result value', 'resCube', 'outcome', 4.0),
    lab(code('n_steps'), 'nPlate', 'state', 1.2),
  ],
];

const SHOT_KEYS = [
  [0, 'IN', 2], [1, 'BED', 1.5], [2, 'RULE', 1.5], [3, 'CALL', 1.5], [4, 'FUEL', 1.5],
  [5, 'TAPE', 1.5], [6, 'COUNT', 1.5], [7, 'INSET', 1.5],
];

export default {
  id: 'run',
  title: 'The run',
  beats: BEATS.map((b, i) => ({
    id: b.id,
    title: b.title,
    duration: b.duration,
    narration: b.narration,
    labels: LABELS[i],
  })),

  build(c) {
    ctx = c;
    const k = ctx.camera.shots;
    C = {
      ceramic: col(PALETTE.ceramic), titanium: col(PALETTE.titanium), gold: col(PALETTE.gold),
      goldB: col(PALETTE.goldBright), ink: col(PALETTE.ink), inkSoft: col(PALETTE.inkSoft),
      on: col(PALETTE.onSurface), floor: col(PALETTE.paper), program: col(FAMILY.program.hex),
      programB: col(FAMILY.program.bright), input: col(FAMILY.input.hex), inputB: col(FAMILY.input.bright),
      outcome: col(FAMILY.outcome.hex), outcomeB: col(FAMILY.outcome.bright), step: col(FAMILY.step.hex),
      stepB: col(FAMILY.step.bright), error: col(FAMILY.error.hex), dim: col(PALETTE.paperWarm),
    };
    geo = new THREE.BoxGeometry(1, 1, 1);
    const matteMat = materials.ceramic();
    matteMat.color.set(1, 1, 1);
    const metalMat = materials.titanium();
    metalMat.color.set(1, 1, 1);
    const litMat = new THREE.MeshBasicMaterial();
    ownedMats = [matteMat, metalMat, litMat];
    pools = {
      matte: new Pool(geo, matteMat, 640),
      metal: new Pool(geo, metalMat, 160),
      lit: new Pool(geo, litMat, 160),
    };
    for (const p of Object.values(pools)) ctx.scene.add(p.mesh);

    toks = [];
    for (let i = 0; i < 6; i++) {
      const t = makeDigestToken({ family: 'state', content: `link:${i}`, size: 0.5 });
      t.rotation.x = -PI / 2;
      ctx.scene.add(t);
      toks.push(t);
    }

    anchors = {};
    for (const n of LABEL_NAMES) {
      const o = new THREE.Object3D();
      ctx.scene.add(o);
      anchors[n] = o;
      ctx.labels.anchor(n, o);
    }

    const shot = {
      IN: k.establish([-10, 1, 0], 30), BED: k.establish([-6, 0.5, 0], 18),
      RULE: k.establish([-1.5, 0.5, -1.5], 24), CALL: k.establish([-9.5, 0.5, 1.5], 16),
      FUEL: k.establish([-2.2, 0.5, -1.5], 9), TAPE: k.establish([6, 0.5, 3.2], 14),
      COUNT: k.establish([9, 0.5, -0.4], 22), INSET: k.establish([11.5, 0.5, -0.6], 13),
      ALL: k.establish([0, 1, 0], 36), OUT: k.establish([10, 1, 0], 30),
    };
    const keys = SHOT_KEYS.map(([bi, name, dur]) => ({ t: S[bi], pose: shot[name], dur }));
    keys.push({ t: S[9], pose: shot.ALL, dur: 1.5 });
    keys.push({ t: S[9] + 10, pose: shot.OUT, dur: 1.5 });
    ctx.camera.track(keys);
  },

  seek(t) {
    T = t;
    const { matte, metal, lit } = pools;
    matte.reset(); metal.reset(); lit.reset();
    setT(0, 0, 0, 0, 1);
    for (const tk of toks) tk.visible = false;

    // ------------------------------------------------------------------ machine, rack, plates
    const slide = W(0, 2, 4);
    const rx = lerp(-20, -14, slide);
    const rz = lerp(0, -1.5, slide);
    drawRack(rx, 1.5, rz);
    box(metal, -14, 0.1, -1.5, 3.4, 0.2, 2.0, C.titanium);                  // ROM socket
    box(metal, -14, 0.05, 1.2, 2.4, 0.1, 1.0, C.titanium);                  // refPlate
    sweepBar(S[0] + 5.0, -15.2, -12.8, 0.2, 1.2, 1.0);
    if (glowing(S[0] + 5.0)) ring(lit, -14, 0.13, 1.2, 2.4, 1.0, 0.06, 0.04, C.goldB);

    // cowl: lifts in beat 2, parked and hidden after
    const lift = W(1, 2, 3.5);
    const cs = 1 - ramp(T, S[1] + 3.1, S[1] + 3.5);
    if (cs > 0.002) {
      setT(-6, lerp(0, 5.2, lift), 0, 0, 1);
      // scale about the centre of the cowl
      const cy = 1.3;
      box(matte, 0, cy * cs, 0, 12 * cs, 2.4 * cs, 6.4 * cs, C.ceramic);
      box(metal, 0, 2.44 * cs, 3.2 * cs, 12 * cs, 0.12 * cs, 0.12 * cs, C.titanium);
      box(metal, 0, 2.44 * cs, -3.2 * cs, 12 * cs, 0.12 * cs, 0.12 * cs, C.titanium);
      box(metal, -6 * cs, 2.44 * cs, 0, 0.12 * cs, 0.12 * cs, 6.4 * cs, C.titanium);
      box(metal, 6 * cs, 2.44 * cs, 0, 0.12 * cs, 0.12 * cs, 6.4 * cs, C.titanium);
      setT(0, 0, 0, 0, 1);
    }

    // bays: floors and rims
    for (const b of BAYS) {
      box(matte, b.cx, 0.05, b.cz, 3.6, 0.1, 2.6, C.floor);
      ring(metal, b.cx, 0.15, b.cz, 3.55, 2.55, 0.1, 0.1, C.titanium);
    }
    bayContents();

    // rule table, counter, ruler
    ruleTable();
    // tape lane and head
    box(lit, 8, 0.025, LANE_Z, 12, 0.05, 0.08, C.on);
    box(metal, 1.5, 0.6 + headDip(), LANE_Z, 1.0, 0.8, 1.4, C.titanium);
    tape();
    counterAndRuler();
    insets();
    sheetAndConnectors();

    matte.done(); metal.done(); lit.done();
    placeAnchors();
  },

  dispose() {
    for (const p of Object.values(pools || {})) { ctx.scene.remove(p.mesh); p.mesh.dispose(); }
    if (geo) geo.dispose();
    for (const m of ownedMats) m.dispose();
    for (const tk of toks || []) {
      ctx.scene.remove(tk);
      tk.traverse((o) => {
        if (o.geometry) o.geometry.dispose();
        if (o.material) (Array.isArray(o.material) ? o.material : [o.material]).forEach((m) => m.dispose());
      });
    }
    for (const a of Object.values(anchors || {})) ctx.scene.remove(a);
    ctx = null; pools = null; geo = null; toks = null; anchors = null; C = null; ownedMats = [];
  },
};

// ---- parts ----------------------------------------------------------------------------------------------------

function drawRack(x, y, z) {
  const { metal, matte } = pools;
  setT(x, y, z, 0, 1);
  for (const sx of [-1.44, 1.44]) for (const sz of [-0.74, 0.74]) box(metal, sx, 0, sz, 0.12, 2.6, 0.12, C.titanium);
  box(metal, 0, 1.26, 0, 3.0, 0.08, 1.6, C.titanium);
  box(metal, 0, -1.26, 0, 3.0, 0.08, 1.6, C.titanium);
  for (let r = 0; r < 5; r++) {
    const sy = 0.9 - 0.45 * r;
    box(matte, 0, sy, 0, 2.6, 0.08, 1.2, C.program);
    for (let j = 0; j < 6; j++) box(matte, -1.0 + 0.4 * j, sy + 0.165, 0, 0.25, 0.25, 0.25, C.programB);
  }
  setT(0, 0, 0, 0, 1);
}

function bayContents() {
  const { matte, metal, lit } = pools;
  const s0 = bayS(0), s1 = bayS(1), s2 = bayS(2), s3 = bayS(3), s4 = bayS(4), s5 = bayS(5);

  // ---- control bay
  const B0 = BAYS[0];
  setT(B0.cx, 0, B0.cz, 0, s0);
  const nextK = ramp(T, S[2] + 5.0, S[2] + 5.6) * (1 - ramp(T, S[3], S[3] + 0.4));
  const leave = T >= S[3] + 2.0;
  for (let j = 0; j < 4; j++) {
    const x = -0.65 + 0.5 * j;
    if (!(j === 3 && leave)) {
      box(matte, x, 0.3, -0.7, 0.4, 0.4, 0.4, C.ceramic);
      box(matte, x, 0.52, -0.7, 0.14, 0.04, 0.14, C.ink);
    }
    box(matte, x, 0.3, 0.2, 0.4, 0.4, 0.4, mix(C.ceramic, C.step, nextK));
  }
  for (let k = 0; k < 4; k++) box(matte, -1.4, 0.225, -0.7 + 0.45 * k, 0.25, 0.25, 0.25, C.programB);
  // result and Boolean cubes of beat 4
  const rb = W(3, 5.3, 5.7) * (1 - W(4, 0, 0.4));
  box(matte, -0.8, 0.3, 0.95, 0.4 * rb, 0.4 * rb, 0.4 * rb, C.ceramic);
  box(matte, 0.6, 0.3, 0.95, 0.4 * rb, 0.4 * rb, 0.4 * rb, C.ceramic);
  box(matte, 0.6, 0.52, 0.95, 0.14 * rb, 0.04 * rb, 0.14 * rb, C.ink);
  setT(0, 0, 0, 0, 1);

  // sweep across current row and ROM row
  sweepBar(S[2] + 2.0, -11.5, -8.5, 0.5, -2.2, 0.7);
  if (glowing(S[2] + 2.0)) {
    ring(lit, -9.4, 0.58, -2.2, 3.0, 0.62, 0.05, 0.04, C.goldB);
  }

  // ---- environment bay
  const B1 = BAYS[1];
  setT(B1.cx, 0, B1.cz, 0, s1);
  for (let j = -1; j <= 1; j++) box(matte, 0.5 * j, 0.3, -0.3, 0.4, 0.4, 0.4, C.ceramic);
  setT(0, 0, 0, 0, 1);

  // ---- frame stack
  const B2 = BAYS[2];
  setT(B2.cx, 0, B2.cz, 0, s2);
  const push = W(3, 2.4, 3.2);
  const nLinks = T >= S[3] + 3.2 ? 4 : 3;
  for (let i = 0; i < 3; i++) {
    const lz = 0.95 - 0.62 * i - 0.62 * push;
    linkTok(i, 0, lz);
    if (i < 2) box(metal, 0, 0.17, lz - 0.31, 0.1, 0.1, 0.12, C.gold);
  }
  if (nLinks === 4) {
    const pop = lerp(0.4, 1, ramp(T, S[3] + 3.2, S[3] + 3.5));
    linkTok(3, 0, 0.95, pop);
    box(metal, 0, 0.17, 0.64, 0.1, 0.1, 0.12, C.gold);
  }
  setT(0, 0, 0, 0, 1);

  // ---- task stack
  const B3 = BAYS[3];
  setT(B3.cx, 0, B3.cz, 0, s3);
  linkTok(4, 0, 0.95);
  linkTok(5, 0, 0.33);
  box(metal, 0, 0.17, 0.64, 0.1, 0.1, 0.12, C.gold);
  setT(0, 0, 0, 0, 1);

  // ---- memory
  const B4 = BAYS[4];
  setT(B4.cx, 0, B4.cz, 0, s4);
  box(matte, 0, 0.12, 0, 3.3, 0.04, 0.6, C.ink);
  ring(metal, 0, 0.16, 0, 3.4, 0.7, 0.08, 0.12, C.titanium);
  for (let j = 0; j < 5; j++) {
    const x = -1.24 + 0.62 * j;
    box(matte, x, 0.19, 0, 0.5, 0.1, 0.5, C.ceramic);
    box(matte, x, 0.25, 0, 0.32, 0.02, 0.06, C.ink);
  }
  setT(0, 0, 0, 0, 1);
  // connector from the environment to the saved input tile
  const lightK = W(3, 6.0, 6.4) * (1 - W(4, 0, 0.4));
  if (s1 > 0.99 && s4 > 0.99) {
    poly(lit, P_ENV, 1, LINE_Y, mix(C.on, C.dim, 0.6 * (1 - lightK)).lerp(C.stepB, lightK * 0.7));
    if (lightK > 0.01) ring(lit, -3.44, 0.27, 1.5, 0.62, 0.62, 0.05, 0.04, mix(C.step, C.stepB, 0.5));
  }

  // ---- fuel
  const B5 = BAYS[5];
  setT(B5.cx, 0, B5.cz, 0, s5);
  const lower = W(4, 4.2, 5.0);
  const remain = 0.75 - 0.125 * lower;
  const work = 0.25 + 0.125 * lower;
  const gx = [-1.3, -0.7, -0.1];
  const fr = [1, remain, work];
  for (let g = 0; g < 3; g++) {
    box(metal, gx[g], 0.15, -0.2, 0.4, 0.1, 1.6, C.titanium);
    const L = 1.5 * fr[g];
    box(matte, gx[g], 0.215, -0.95 + L / 2, 0.28, 0.04, L, C.ceramic);
  }
  box(matte, 0.7, 0.3, 0, 0.4, 0.4, 0.4, C.ceramic);
  box(matte, 1.2, 0.3, 0, 0.4, 0.4, 0.4, C.ceramic);
  setT(0, 0, 0, 0, 1);
  // limit cells: gold outline for 0.3 s
  if (IN(4, 4.6, 4.9)) {
    ring(lit, -1.5, 0.52, -1.5, 0.5, 0.5, 0.05, 0.04, C.gold);
    ring(lit, -1.0, 0.52, -1.5, 0.5, 0.5, 0.05, 0.04, C.gold);
  }
  // the charge cube
  const drop = W(4, 2.0, 3.0);
  const cg = (1 - W(5, 0, 0.4)) * (T >= S[4] + 2.0 ? 1 : 0);
  if (cg > 0.01) {
    const cy = lerp(5, 0.3, drop * drop);
    const k = 0.4 * cg;
    box(matte, -2.9, cy, -0.4, k, k, k, C.step);
    sweepBar(S[4] + 3.5, -3.7, -2.1, 0.55, -1.2, 2.2);
    if (glowing(S[4] + 3.5)) {
      ring(lit, -2.9, 0.5, -0.4, 0.5, 0.5, 0.05, 0.04, C.goldB);
      ring(lit, -2.9, 0.27, -1.7, 0.5, 1.7, 0.05, 0.04, C.goldB);
    }
  }
}

function linkTok(i, lx, lz, pop = 1) {
  const t = toks[i];
  tw(lx, 0.17, lz, WP);
  t.position.copy(WP);
  t.scale.setScalar(Math.max(0.001, TX.s * pop));
  t.visible = TX.s > 0.01;
}

function ruleTable() {
  const { matte, lit } = pools;
  const k = W(2, 0, 0.4);
  if (k < 0.01) return;
  setT(3.4, 0, -2.5, 0, k);
  for (let r = 0; r < KEY_ROWS; r++) {
    for (let c = 0; c < KEY_COLS; c++) {
      if (c === HOT.c && r === HOT.r) continue;
      box(matte, (c - 5.5) * 0.6, 0.1, (r - 4.5) * 0.5, 0.5, 0.12, 0.4, C.ceramic);
    }
  }
  const up = W(2, 3.0, 3.4);
  const hc = mix(C.step, C.stepB, 0.3 * up);
  box(up > 0.5 ? lit : matte, (HOT.c - 5.5) * 0.6, 0.1 + 0.15 * up, (HOT.r - 4.5) * 0.5, 0.5, 0.12, 0.4, up > 0.5 ? hc : C.ceramic);
  setT(0, 0, 0, 0, 1);
}

function counterAndRuler() {
  const { matte, metal, lit } = pools;
  const ck = W(6, 0.5, 0.9);
  if (ck > 0.01) {
    setT(10.5, 0, -4.6, 0, ck);
    box(metal, 0, 0.06, 0, 1.9, 0.12, 0.3, C.titanium);
    for (let j = 0; j < 8; j++) {
      let c = C.ceramic;
      if (j === 7) {
        let l = 0;
        for (let p = 1; p <= 3; p++) {
          const r = ramp(T, PRINTS[p].d, PRINTS[p].d + 0.4);
          if (r > 0 && r < 1) l = 1;
        }
        if (l) c = C.step;
      }
      box(matte, (j - 3.5) * 0.22, 0.22, 0, 0.2, 0.2, 0.2, c);
    }
    setT(0, 0, 0, 0, 1);
  }
  const rk = W(6, 7.0, 7.4);
  if (rk > 0.01) {
    setT(11.75, 0, -3.6, 0, rk);
    box(metal, 0, 0.05, 0, 6.5, 0.1, 0.3, C.titanium);
    for (let i = 0; i <= 8; i++) box(lit, -3.25 + 0.5 * i, 0.15, 0, 0.04, 0.1, 0.14, C.on);
    box(lit, 0.63, 0.15, 0, 0.04, 0.1, 0.5, C.on, 0.35);
    box(lit, 0.83, 0.15, 0, 0.04, 0.1, 0.5, C.on, 0.35);
    for (let i = 0; i < 3; i++) box(lit, 1.95 + 0.5 * i, 0.15, 0, 0.04, 0.1, 0.14, C.on);
    box(lit, 3.25, 0.22, 0, 0.07, 0.25, 0.3, C.on);
    setT(0, 0, 0, 0, 1);
  }
}

function tape() {
  const { matte, metal, lit } = pools;
  const e = W(9, 12, 14.2);
  const gx = lerp(GROUP_BASE_X, 20, e);
  const gz = lerp(LANE_Z, 0, e);
  const gy = 0.2 * e;
  const cl = W(9, 4, 6);
  const dropOut = 1 - W(9, 4, 4.6);
  const stateWithout = {};
  for (let idx = 0; idx < FRAMES.length; idx++) {
    const f = FRAMES[idx];
    let s;
    let x = frameX(f);
    if (idx < 2) {
      s = W(5, 0.8, 1.2);
      if (s < 0.01) continue;
    } else {
      const p = PRINTS[f.own];
      if (T < p.d) continue;
      s = lerp(0.3, 1, W(0, 0, 0) * 0 + ramp(T, p.d, p.d + 0.6));
    }
    let y = 0;
    let ry = 0;
    let z = LANE_Z;
    if (f.kept) {
      if (cl > 0) {
        const tx = gx + f.loc;
        x = lerp(x, tx, cl);
        z = lerp(LANE_Z, gz, cl);
        y = 0.9 * Math.sin(PI * cl) + (cl >= 1 ? gy : 0);
        ry = PI * cl;
      }
      if (cl >= 1) { x = gx + f.loc; z = gz; y = gy; }
    } else {
      s *= dropOut;
      if (s < 0.01) continue;
    }
    const o = { rule: f.id === 'G' ? C.outcome : C.step };
    if (f.id === 'G') o.cube = W(9, 2.0, 3.2) * 1;
    if (f.id === 'C') {
      const eq = ramp(T, S[5] + 3.6, S[5] + 4.6);
      const back = W(6, 0, 0.4);
      o.beforeOff = 0.3 * phase(T, S[5] + 3.6, S[5] + 4.6);
      o.beforeS = T > S[5] + 4.6 && T < S[6] + 0.4 ? 0 : (T >= S[6] + 0.4 ? 1 : 1);
      if (T >= S[6]) o.beforeOff = 0.3 * (1 - back);
      if (T >= S[6] && T < S[6] + 0.4) o.beforeS = back;
      if (eq >= 1 && T < S[6]) o.beforeS = 0;
    }
    drawFrame(f, x, y, z, ry, s, o);
  }
  // equality outline around the shared boundary cells
  const ok = (T >= S[5] + 4.6 ? 1 : 0) * (1 - W(6, 0, 0.4));
  if (ok > 0.01 && FR_STATE.B) {
    const xb = frameX(FRAMES[1]) - 0.3 + 0.0;
    const c = IN(5, 4.6, 4.9) ? C.goldB : C.gold;
    setT(0, 0, 0, 0, 1);
    ring(lit, xb, 0.3, LANE_Z, 0.18 * 1.1 + 0.1, 0.9 * 1.1, 0.05 * ok, 0.04 * ok, c);
  }
  // connector from the firing rule to the new frame
  const cf = W(5, 5.0, 6.0) * (1 - W(6, 0, 0.4));
  poly(lit, P_TAPE, cf, LINE_Y, C.stepB);
  // break mark on the lane and the count plate
  const bm = W(6, 6.2, 6.6);
  const bk = bm * (T >= S[9] + 4 ? 0 : 1);
  const closeK = cl;
  const bx = lerp(9.5, gx + BREAK_LOC, closeK);
  const bz = lerp(LANE_Z, gz, closeK);
  const by = (closeK >= 1 ? gy : 0) + 0.9 * Math.sin(PI * closeK) * 0;
  if (bm > 0.01) {
    box(lit, bx - 0.09, 0.1 + by, bz, 0.05 * bm, 0.08, 0.7 * bm, C.on, 0.35);
    box(lit, bx + 0.09, 0.1 + by, bz, 0.05 * bm, 0.08, 0.7 * bm, C.on, 0.35);
  }
  void bk;
  const pn = W(6, 6.0, 6.5);
  if (pn > 0.01) {
    const nx = lerp(14.6, gx + NPLATE_LOC, closeK);
    const nz = lerp(LANE_Z, gz, closeK);
    const ny = (closeK >= 1 ? gy : 0) + 0.9 * Math.sin(PI * closeK);
    box(metal, nx, 0.1 + ny, nz, 0.6 * pn, 0.2 * pn, 0.6 * pn, C.gold);
    FP.set(nx, 0.2 + ny, nz);
    NP.copy(FP);
  }
  void stateWithout; void matte;
}
const NP = new THREE.Vector3(14.6, 0.3, LANE_Z);

function insets() {
  const { matte, metal, lit } = pools;
  const pA = W(7, 0, 0.6) * (1 - W(9, 0, 0.6));
  const pB = W(8, 0, 0.6) * (1 - W(9, 0, 0.6));
  insetFrame(9.3, pA);
  insetFrame(13.6, pB);

  // ---- tray A: the overflow
  if (pA > 0.01) {
    setT(9.3, -(1 - pA) * 0.4, -0.6, 0, pA);
    box(matte, -1.3, 0.3, -0.6, 0.4, 0.4, 0.4, C.ceramic);
    box(matte, -0.6, 0.3, -0.6, 0.4, 0.4, 0.4, C.ceramic);
    box(matte, 0.3, 0.12, 1.0, 0.9, 0.06, 1.2, C.ceramic);        // empty frame plate
    const dz = 1.1 * W(7, 2.0, 2.3) - 0.4 * W(7, 2.3, 2.6);
    box(matte, 0.3, 0.3, -0.6 + dz, 0.4, 0.4, 0.4, C.ceramic);
    const hk = W(7, 2.6, 2.8);
    hatch(0.3, 0.52, -0.6 + dz, hk);
    const tk = W(7, 3.4, 3.8);
    if (tk > 0.01) {
      box(matte, 0.3, 0.19, 1.0, 0.3 * tk, 0.08 * tk, 0.5 * tk, C.error);
      box(matte, 0.65, 0.2, 1.0, 0.2 * tk, 0.2 * tk, 0.2 * tk, C.inkSoft);
    }
    tw(0.3, 0.5, -0.6 + dz, WP); anchors.ovf.position.copy(WP);
    tw(0.3, 0.3, 1.0, WP); anchors.rejTag.position.copy(WP);
    tw(0.65, 0.35, 1.0, WP); anchors.rejPay.position.copy(WP);
    setT(0, 0, 0, 0, 1);
  }
  // ---- tray B: the fuel shortfall
  if (pB > 0.01) {
    setT(13.6, -(1 - pB) * 0.4, -0.6, 0, pB);
    box(metal, 0.5, 0.15, -0.6, 2.0, 0.1, 0.4, C.titanium);
    box(matte, -0.4, 0.215, -0.6, 0.2, 0.04, 0.28, C.ceramic);
    box(matte, 0.5, 0.12, 1.0, 0.9, 0.06, 1.2, C.ceramic);
    const dx = 0.7 * W(8, 2.0, 2.3) - 0.4 * W(8, 2.3, 2.6);
    box(matte, -1.7 + dx, 0.3, -0.6, 0.4, 0.4, 0.4, C.step);
    hatch(-1.7 + dx, 0.52, -0.6, W(8, 2.6, 2.8));
    const tk = W(8, 3.4, 3.8);
    if (tk > 0.01) {
      box(matte, 0.5, 0.19, 1.0, 0.3 * tk, 0.08 * tk, 0.5 * tk, C.error);
      box(matte, 0.85, 0.2, 1.0, 0.2 * tk, 0.2 * tk, 0.2 * tk, C.inkSoft);
    }
    tw(-1.7 + dx, 0.5, -0.6, WP); anchors.chargeB.position.copy(WP);
    tw(0.5, 0.25, -0.6, WP); anchors.gaugeB.position.copy(WP);
    tw(0.5, 0.3, 1.0, WP); anchors.exhTag.position.copy(WP);
    tw(0.85, 0.35, 1.0, WP); anchors.exhPay.position.copy(WP);
    setT(0, 0, 0, 0, 1);
  }
  void lit;
}

function insetFrame(cx, p) {
  if (p < 0.01) return;
  const { matte, metal } = pools;
  setT(cx, -(1 - p) * 0.4, -0.6, 0, 1);
  box(matte, 0, 0.05, 0, 4.2, 0.1, 4.0, C.floor);
  ring(metal, 0, 0.15, 0, 4.15, 3.95, 0.12, 0.1, C.titanium);
  setT(0, 0, 0, 0, 1);
}

// Refusal hatch: three bars at 45 degrees across a top face.
function hatch(x, y, z, k) {
  if (k < 0.01) return;
  const { lit } = pools;
  const lens = [0.5, 0.32, 0.32];
  const offs = [0, -0.12, 0.12];
  for (let i = 0; i < 3; i++) {
    const ox = offs[i] * 0.7071;
    box(lit, x + ox, y, z + ox, 0.08 * k, 0.02, lens[i] * k, C.error, PI / 4);
  }
}

function sheetAndConnectors() {
  const { matte, lit } = pools;
  const vis = W(3, 0, 0.4) * (1 - W(4, 0, 0.4));
  if (vis > 0.01) {
    setT(-14.2, 0, 4.2, 0, vis);
    box(matte, 0, 0.04, 0, 2.0, 0.04, 2.7, C.ceramic);
    for (let k = 0; k < 12; k++) {
      if (k === 2) continue;
      const len = BAR_LEN[k] * 0.5;
      box(matte, -0.85 + len / 2, 0.08, 0.5 * (-2.3 + 0.42 * k), len, 0.04, 0.12, C.program);
    }
    setT(0, 0, 0, 0, 1);
  }
  const g = 1 - W(4, 0, 0.4);
  const f1 = W(3, 4.0, 5.5) * g;
  poly(lit, P_CALL, f1, LINE_Y, C.on);
  poly(lit, P_ADD, f1, LINE_Y, C.on);
  poly(lit, P_BOOL, f1, LINE_Y, C.on);
  // next-state connector of beat 3
  const fn = W(2, 4.0, 5.0) * (1 - W(3, 0, 0.4));
  poly(lit, P_NEXT, fn, LINE_Y, C.stepB);

  // the cell that leaves the control bay and becomes the new cached top
  const mv = W(3, 2.0, 3.2);
  if (mv > 0 && mv < 1) {
    const x = lerp(-8.95, -9.8, mv);
    const z = lerp(-2.2, 2.45, mv);
    const y = 0.3 + 1.4 * Math.sin(PI * mv);
    box(matte, x, y, z, 0.4, 0.4, 0.4, C.ceramic);
  }
  // outline on the active call expression bar
  if (T >= S[3] + 4.0 && T < S[4]) {
    // the call bar lights as its connector arrives
  }
  void lit;
}

function placeAnchors() {
  const a = anchors;
  a.housing.position.set(-6, 2.5, 0);
  a.socket.position.set(-14, 2.8, -1.5);
  a.refPlate.position.set(-14, 0.15, 1.2);
  a.tapePath.position.set(2.0, 0.1, LANE_Z);
  a.bCtrl.position.set(-9.8, 0.5, -1.8);
  a.bEnv.position.set(-6, 0.5, -1.8);
  a.bFrame.position.set(-9.8, 0.4, 1.5);
  a.bTask.position.set(-6, 0.4, 1.5);
  a.bMem.position.set(-2.2, 0.4, 1.5);
  a.bFuel.position.set(-2.2, 0.4, -1.5);
  a.rules.position.set(6.4, 0.2, -3.4);
  a.curState.position.set(-9.45, 0.5, -2.2);
  a.romRow.position.set(-11.2, 0.4, -2.2);
  a.hotKey.position.set(HOT_X, 0.3, HOT_Z);
  a.nextState.position.set(-9.45, 0.5, -1.3);
  a.callExpr.position.set(barRight(K_CALL) - 0.3, 0.2, barZ(K_CALL));
  a.pushed.position.set(-9.8, 0.35, 2.45);
  a.addRes.position.set(-10.6, 0.5, -0.55);
  a.boolRes.position.set(-9.2, 0.5, -0.55);
  a.savedIn.position.set(-3.44, 0.3, 1.5);
  a.gInit.position.set(-3.5, 0.2, -2.4);
  a.gCharge.position.set(-2.9, 0.5, -0.4);
  a.gRemain.position.set(-2.9, 0.2, -2.4);
  a.gWork.position.set(-2.3, 0.2, -2.4);
  a.gLimits.position.set(-1.25, 0.5, -1.5);
  if (FR_STATE.C) {
    frameWorld('C', 0, 0.2, 0, a.ruleId.position);
    frameWorld('C', -0.3, 0.3, 0, a.stAfter.position);
    frameWorld('B', 0.3, 0.3, 0, a.stBefore.position);
    a.eqCells.position.set(frameX(FRAMES[1]) - 0.3, 0.3, LANE_Z);
  }
  a.haltTape.position.set(7.5, 0.1, LANE_Z);
  if (FR_STATE.G) {
    frameWorld('G', 0, 0.2, 0, a.haltTag.position);
    frameWorld('G', -0.3, 0.5, 0, a.resCube.position);
    if (T >= S[9] + 6) a.haltTape.position.set(frameWorld('A', 0, 0.1, 0, WP).x, WP.y, WP.z);
  }
  a.nPlate.position.copy(NP);
  a.counter.position.set(10.5, 0.4, -4.6);
  a.limit.position.set(15.0, 0.4, -3.6);
  a.rulerA.position.set(12.2, 0.3, -3.6);
}
