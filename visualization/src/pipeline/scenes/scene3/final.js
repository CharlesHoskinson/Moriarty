// Slabs and dock, rules hash, the inset trays (beats 3.25, 3.31 to 3.35).
import * as THREE from 'three/webgpu';
import { PALETTE, FAMILY, lerp } from '../../theme.js';
import { makeProofSlab } from '../../kit.js';
import * as D from './draw.js';
import { T, ph, rp, START } from './time.js';
import { lit } from './lit.js';
import { hashPh, eq } from './motion.js';
import { sliceFrameX } from './tape.js';
import { LID_Y } from './tape.js';
import { regX } from './core.js';
import { winX } from './mem.js';

let slabs = [];
const PIN_KINDS = ['collar', 'collar', 'capcollar', 'capcollar', 'collar', 'bar', 'bar'];
const PIN_FAM = ['state', 'program', 'state', 'state', 'state', 'state', 'state'];

export function buildSlabs(scene) {
  const base = makeProofSlab({ variant: 'leaf', pinKinds: PIN_KINDS });
  base.userData.setState({ pins: PIN_FAM });
  slabs = [base, base.clone(), base.clone()];
  for (const s of slabs) { s.visible = false; scene.add(s); }
}
export function disposeSlabs() {
  if (slabs[0]) slabs[0].userData.dispose();
  for (const s of slabs) s.removeFromParent();
  slabs = [];
}
export const slabAnchor = (i, n) => slabs[i].userData.pinAnchors?.[n] ?? slabs[0].userData.pinAnchors[n];

// Dock stack: three slabs, genesis lowest, offsets scaled to stay inside the station.
const DOCK = [[14.0, 0.2, -4.3], [14.4, 0.6, -4.75], [14.8, 1.0, -5.2]];
const OUTP = [[19.2, 0.2, 0.9], [20, 0.6, 0], [20.8, 1.0, -0.9]];
const PIN_X = (n) => -1.3 + (n * 2.6) / 6;
const TOK = [['state:init', 'state:0'], ['state:0', 'state:1'], ['state:1', 'state:final']];
const P = [0, 0, 0];

// Slab pose of slab i as a function of time.
function slabPose(i, out) {
  const bi = T.b;
  let k = 0;
  let pos = DOCK[i];
  if (i === 0) k = bi >= 32 ? 1 : bi === 31 ? ph(31, 3.4, 4.4) : 0;
  if (i > 0) k = bi === 34 ? ph(34, 2.0 + 0.6 * (i - 1), 4.0 + 0.6 * (i - 1)) : bi > 34 ? 1 : 0;
  let m = 0;
  if (bi === 34) m = ph(34, 10.0, 12.4);
  out[0] = lerp(pos[0], OUTP[i][0], m);
  out[1] = lerp(pos[1], OUTP[i][1], m);
  out[2] = lerp(pos[2], OUTP[i][2], m);
  return k;
}

const PIN_POS = [0, 0, 0];
// Where pin n of slab i sits in scene space (for tokens that settle on pins).
function pinWorld(i, n, out) {
  slabPose(i, P);
  out[0] = P[0] + PIN_X(n);
  out[1] = P[1] + 0.5;
  out[2] = P[2] + 1.15;
  return out;
}

export function drawDock() {
  const bi = T.b;
  for (let i = 0; i < 3; i++) {
    const k = slabPose(i, P);
    const s = slabs[i];
    s.visible = k > 0.01;
    s.position.set(P[0], P[1], P[2]);
    s.scale.set(1, Math.max(k, 0.001), 1);
    if (i === 0) {
      const v = bi === 31 ? rp(31, 5.0, 5.7) : 0;
      slabs[0].userData.setState({ pins: PIN_FAM, verify: v, edgeLit: bi === 31 && v >= 1 ? 1 - rp(31, 5.7, 6.0) : 0 });
    }
  }
  D.dim(0);
  // Rules digest token and the two integer bars (beats 3.31, 3.32), moving onto the pins in beat 3.32.
  if (bi >= 30 && bi <= 31) {
    const born = bi === 30 ? ph(30, 4.8, 5.6) : 1;
    const mv = bi === 31 ? ph(31, 4.2, 4.9) : 0;
    pinWorld(0, 4, PIN_POS);
    const rx = lerp(14.4, PIN_POS[0], mv);
    const ry = lerp(0.3, 0.75, mv);
    const rz = lerp(-5.0, PIN_POS[2], mv);
    if (bi === 31 && mv >= 1) { /* settled on pin */ }
    D.token(rx, ry, rz, 0.8 * (0.2 + 0.8 * born) * (1 - 0.4 * mv), 'state', 'rules_d', { k: born > 0.001 ? 1 : 0 });
    const bk = bi === 30 ? ph(30, 5.0, 5.6) : 1;
    for (let b = 0; b < 2; b++) {
      pinWorld(0, 5 + b, PIN_POS);
      const bx = lerp(14.0 + 1.1 * b, PIN_POS[0], mv);
      const bz = lerp(-3.0, PIN_POS[2], mv);
      const by = lerp(0.2, 0.62, mv);
      D.box('matte', bx, by, bz, 0.9 * bk * (1 - 0.6 * mv), 0.15 * bk, 0.3 * bk, PALETTE.ceramic);
      for (let g = 0; g < 4; g++) D.box('matte', bx - 0.3 + g * 0.2, by + 0.09 * bk, bz, 0.03, 0.02, 0.2 * (1 - 0.6 * mv), PALETTE.ink);
    }
    if (bi === 30) D.sweepX(13.4, 15.6, 0.45, -3.0, 0.4, rp(30, 6.4, 7.4));
  }
  if (bi === 34) {
    // Boundary tokens above each slab and its two integer bars.
    for (let i = 0; i < 3; i++) {
      const k = slabPose(i, P);
      const tk = i === 0 ? 1 : ph(34, 4.0 + 0.6 * (i - 1), 4.6 + 0.6 * (i - 1));
      if (k < 0.99 || tk <= 0.01) continue;
      D.token(P[0] - 1.1, P[1] + 1.2, P[2], 0.5, 'state', TOK[i][0], { ring: 1, k: tk });
      D.token(P[0] + 1.1, P[1] + 1.2, P[2], 0.5, 'state', TOK[i][1], { ring: 1, k: tk });
      for (let b = 0; b < 2; b++) D.box('matte', P[0] - 0.5 + b * 1.0, P[1] + 0.55, P[2] + 0.5, 0.7 * tk, 0.12 * tk, 0.26 * tk, PALETTE.ceramic);
    }
  }
  if (bi >= 32 && bi < 34) {
    // Slab 0 keeps its seven pins; the tokens of beat 3.31 sit on their pins.
    pinWorld(0, 4, PIN_POS);
    D.token(PIN_POS[0], 0.75, PIN_POS[2], 0.48, 'state', 'rules_d');
    for (let b = 0; b < 2; b++) {
      pinWorld(0, 5 + b, PIN_POS);
      D.box('matte', PIN_POS[0], 0.62, PIN_POS[2], 0.36, 0.15, 0.3, PALETTE.ceramic);
    }
  }
  if (bi === 34) {
    pinWorld(0, 4, PIN_POS);
  }
}

// Rules hash (beat 3.31): block at (11.6, 0, -5.0), plates copied from the terminal slice.
export function drawRules() {
  const bi = T.b;
  if (bi !== 30) return;
  D.dim(1 - lit('ribbon'));
  const h = hashPh(30, 2.6);
  const bk = ph(30, 2.0, 2.6);
  D.hashBlock(11.6, 0.0, -5.0, bk, h.glow);
  const lift = ph(30, 2.0, 2.6);
  for (let i = 0; i < 16; i++) {
    const x = sliceFrameX('T', i);
    const a = 3.2 + 0.075 * i;
    const g = ph(30, a, a + 0.6);
    if (T.tb < 2.0 || g >= 1) continue;
    const px = lerp(x, 10.4, g);
    const pz = lerp(-5.3, -5.0, g);
    const py = lerp(0.2 + 0.3 * lift, 0.5, g);
    const sc = 1 - g;
    D.box('matte', px, py, pz, 0.3 * sc, 0.08 * sc, 0.5 * sc, i < 10 ? (i === 9 ? PALETTE.resolve : PALETTE.heat) : PALETTE.ceramic);
  }
  D.dim(0);
}

// Inset trays (beat 3.25 constructor checks, beat 3.33 partition comparison).
export const INSET_Y = 1.6;
export function drawInset() {
  const bi = T.b;
  if (bi !== 24 && bi !== 32) return;
  const up = bi === 24 ? ph(24, 0, 0.6) : ph(32, 0, 0.6);
  const y = lerp(LID_Y + 0.1, INSET_Y, up);
  D.dim(0);
  D.box('matte', 3, y, 0.6, 12 * up, 0.1, 5.5 * up, PALETTE.paperWarm);
  D.box('ti', 3, y + 0.04, -2.1 * up + 0.6 * (1 - up) + 0.0, 12 * up, 0.08, 0.1, PALETTE.titanium);
  D.box('ti', 3, y + 0.04, 0.6 + 2.75 * up, 12 * up, 0.08, 0.1, PALETTE.titanium);
  D.box('ti', -3, y + 0.04, 0.6, 0.1, 0.08, 5.5 * up, PALETTE.titanium);
  D.box('ti', 9, y + 0.04, 0.6, 0.1, 0.08, 5.5 * up, PALETTE.titanium);
  const top = y + 0.05;
  if (up < 0.9) return;
  if (bi === 24) drawStructured(top);
  else drawPartition(top);
}

function drawStructured(top) {
  const bi = 24;
  const t = T.tb;
  const g = (a, b) => (t >= a && t < b ? 0.9 : 0);
  // Left half: constructor record with a metadata cell, child tokens and progress cells.
  D.tile(-1.6, top, -1.3, 1, PALETTE.ceramic, true);
  D.fcell(-0.8, top, -1.3, 0.25, 'step', g(2.0, 2.6), PALETTE.ceramic);
  for (let i = 0; i < 3; i++) D.token(-2.4 + 0.8 * i, top, -0.1, 0.6, 'state', 'child:' + i, { lit: g(2.6 + 0.4 * i, 3.2 + 0.4 * i) });
  for (let i = 0; i < 3; i++) D.fcell(-2.1 + 0.5 * i, top, 1.1, 0.3, 'step', g(3.8, 4.1), PALETTE.ceramic);
  D.cell(-1.6, top, 2.2, 0.35);
  const cmp = ph(24, 4.0, 4.4);
  if (cmp > 0.5) D.outline(-1.6, top + 0.2, 2.2, 0.35, 0.35, PALETTE.gold, 0.05, 0.3);
  D.sweepZ(-1.9, 2.5, top + 0.4, -1.6, 3.4, rp(24, 2.0, 4.4));
  D.line(-1.6, -1.0, -1.6, -0.4, 1, top + 0.03, PALETTE.onSurface, 0.04);
  D.line(-1.6, 0.3, -1.6, 0.9, 1, top + 0.03, PALETTE.onSurface, 0.04);
  D.line(-1.6, 1.4, -1.6, 2.0, 1, top + 0.03, PALETTE.onSurface, 0.04);
  // Right half: two arrangements of the same value, each with a value token.
  const arr = [[-0.8, [3.4, 4.2, 5.0]], [1.6, [3.9, 4.9, 6.0]]];
  arr.forEach(([z, xs], a) => {
    xs.forEach((x) => D.tile(x, top, z, 0.8, PALETTE.ceramic, true));
    D.line(7.6, z, xs[2], z, 1, top + 0.03, PALETTE.onSurface, 0.04);
    if (a === 0) D.line(7.6, z + 0.15, xs[2], z + 0.15, 1, top + 0.03, PALETTE.onSurface, 0.04);
  });
  const e = eq(24, 5.0);
  D.token(7.6, top, -0.8, 0.6, 'state', 'vd:same');
  if (e.show) D.token(7.6, top, lerp(1.6, -0.8, e.m), 0.6, 'state', 'vd:same');
  if (e.ring) D.ring(7.6, top + 0.1, -0.8, 0.6, 0.6, e.lit);
}

function drawPartition(top) {
  const tracks = [[-0.6, [15.5]], [1.4, [9.5, 16.5]]];
  const px = (i) => -2.5 + 0.36 * i;
  tracks.forEach(([z, cuts], a) => {
    for (let i = 0; i < 24; i++) {
      D.box('matte', px(i), top + 0.03, z, 0.32, 0.06, 0.42, PALETTE.ceramic);
      D.box('matte', px(i), top + 0.08, z, 0.1, 0.04, 0.18, PALETTE.heat);
    }
    for (const c of cuts) D.box('ti', px(c + 0.0) + 0.0, top + 0.2, z, 0.08, 0.4, 0.55, PALETTE.titanium);
    // Tray at scale 0.3 holding five tiles, two segment tokens and the final memory token.
    D.box('ti', 7.5, top + 0.03, z, 2.2, 0.05, 0.4, PALETTE.titanium);
    for (let i = 0; i < 5; i++) D.tile(6.65 + i * 0.27, top + 0.04, z, 0.28, PALETTE.ceramic, true);
    D.token(6.9, top, z + 0.65, 0.3, 'state', 'seg:a');
    D.token(7.4, top, z + 0.65, 0.3, 'state', 'seg:b');
    // Fuel gauge icon at the same level on both tracks.
    const gk = ph(32, 4.0, 4.6);
    D.box('ti', -1.4, top + 0.05, z - 0.55, 1.0 * gk, 0.08, 0.2, PALETTE.titanium);
    D.box('gold', -1.65 + 0.3 * gk, top + 0.1, z - 0.55, 0.6 * gk, 0.06, 0.14, PALETTE.gold);
  });
  const e = eq(32, 2.0);
  D.token(8.5, top, -0.6, 0.45, 'state', 'mem:final');
  if (e.show) D.token(8.5, top, lerp(1.4, -0.6, e.m), 0.45, 'state', 'mem:final');
  if (e.ring) D.ring(8.5, top + 0.1, -0.6, 0.45, 0.45, e.lit);
}

// Beat 3.34: cost plates, bracket, broken tape and the flat depth ruler on the lid.
export function drawCost() {
  const bi = T.b;
  if (bi < 33) return;
  D.dim(0);
  if (bi === 33) {
    const pl1 = ph(33, 2.8, 3.4);
    const pl2 = ph(33, 4.0, 4.6);
    D.sweepX(-8, -5.85, 0.3, 5.75, 0.3, rp(33, 2.0, 2.8));
    D.sweepX(-5.85, -4.69, 0.3, 5.75, 0.3, rp(33, 3.2, 4.0));
    D.box('ti', -6.9, 0.25 + 0.5 * pl1, 5.75, 1.4, 0.1, 0.6, PALETTE.titanium);
    D.box('ti', -5.3, 0.25 + 0.5 * pl2, 5.75, 1.4, 0.1, 0.6, PALETTE.titanium);
    const bk = ph(33, 5.0, 5.8);
    D.box('ti', 0, 0.1, 5.35, 16 * bk, 0.05, 0.06, PALETTE.titanium);
    D.box('ti', -8, 0.1, 5.5, 0.06, 0.05, 0.3, PALETTE.titanium);
    D.box('ti', 8, 0.1, 5.5, 0.06, 0.05, 0.3, PALETTE.titanium);
    D.box('matte', -12, 0.06, -5.3, 8, 0.04, 0.3, PALETTE.ceramic);
    D.breakMark(-12, 0.1, -5.3, 0.5);
    D.box('ti', 9.5, LID_Y + 0.15, -2.0, 4, 0.06, 0.2, PALETTE.titanium);
    for (let i = 0; i < 6; i++) D.box('matte', 7.7 + i * 0.32, LID_Y + 0.2, -2.0, 0.04, 0.03, 0.16, PALETTE.onSurface);
    D.breakMark(9.5, LID_Y + 0.2, -2.0, 0.3);
  }
}
