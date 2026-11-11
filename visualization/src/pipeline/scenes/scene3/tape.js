// Tape side of the plan: lid, key rack, lane, ribbon, cutter and the three slices (beats 3.1, 3.4, 3.29, 3.31).
import { PALETTE, lerp } from '../../theme.js';
import * as D from './draw.js';
import { T, START, ph, rp, after, win } from './time.js';
import { lit } from './lit.js';

export const LID_Y = 0.55;
// ---- slice locations ----
const LOC = {
  R: { x0: -7.5, z: -5.3, p: 1, s: 1, y: 0.05 },
  W1: { x0: -15.4, z: -5.3, p: 0.4, s: 0.4, y: 0.03 },
  W2: { x0: -15.4, z: -3.9, p: 0.4, s: 0.4, y: 0.03 },
};
// Swaps: the named slice enters the ribbon, the current ribbon slice takes its place (1.2 s).
const SWAPS = [
  { b: 3, inc: 'T', out: 'A' },
  { b: 4, inc: 'A', out: 'T' },
  { b: 28, inc: 'G', out: 'A' },
  { b: 30, inc: 'T', out: 'G' },
];
const INIT = { A: 'R', T: 'W1', G: 'W2' };
const cur = { x0: 0, z: 0, p: 1, s: 1, y: 0 };

function sliceLoc(name) {
  let loc = INIT[name];
  for (const e of SWAPS) {
    if (name !== e.inc && name !== e.out) continue;
    // Locations are tracked by name: the incoming slice takes the place the outgoing one leaves.
    const q = ph(e.b, 0, 1.2);
    const other = name === e.inc ? e.out : e.inc;
    const to = locAfter(other, e.b);
    if (T.t < START[e.b]) break;
    if (T.t < START[e.b] + 1.2) {
      const a = LOC[loc];
      const b = LOC[to];
      cur.x0 = lerp(a.x0, b.x0, q); cur.z = lerp(a.z, b.z, q); cur.p = lerp(a.p, b.p, q);
      cur.s = lerp(a.s, b.s, q); cur.y = lerp(a.y, b.y, q) + Math.sin(Math.PI * q) * 0.5;
      return cur;
    }
    loc = to;
  }
  const a = LOC[loc];
  cur.x0 = a.x0; cur.z = a.z; cur.p = a.p; cur.s = a.s; cur.y = a.y;
  return cur;
}
// Location of slice `name` just before swap event at beat b (by replaying earlier swaps).
function locBefore(name, b) {
  let loc = INIT[name];
  for (const e of SWAPS) {
    if (e.b >= b) break;
    if (name === e.inc) loc = 'R';
    else if (name === e.out) loc = locBefore(e.inc, e.b);
  }
  return loc;
}
function locAfter(name, b) {
  // Where `name` sits before the swap at beat b; the other slice in the swap moves there.
  return locBefore(name, b);
}

// ---- one witness frame (kit: plate 0.9 by 0.06 by 1.2, rule plate, six cubes) ----
function frame(x, y, z, s, kind, active) {
  D.box('matte', x, y + 0.03 * s, z, 0.9 * s, 0.06 * s, 1.2 * s, PALETTE.ceramic);
  const rule = kind === 'outcome' ? PALETTE.resolve : active ? PALETTE.heat : PALETTE.ceramic;
  D.box('matte', x, y + 0.1 * s, z, 0.3 * s, 0.08 * s, 0.5 * s, rule);
  const cs = 0.18 * s;
  for (let j = -1; j <= 1; j++) {
    D.box('matte', x - 0.3 * s, y + 0.06 * s + cs / 2, z + j * 0.36 * s, cs, cs, cs, PALETTE.ceramic);
    D.box('matte', x + 0.3 * s, y + 0.06 * s + cs / 2, z + j * 0.36 * s, cs, cs, cs, active ? PALETTE.ceramicEdge : PALETTE.ceramic);
  }
}

// Draws a slice. Terminal slice T has 10 active frames, the tenth with an outcome plate.
export function drawSlice(name, k, extra = 0) {
  if (k <= 0.001) return;
  const L = sliceLoc(name);
  const d0 = D.getDim();
  for (let i = 0; i < 16; i++) {
    const inactive = name === 'T' && i >= 10;
    const kind = name === 'T' && i === 9 ? 'outcome' : 'rule';
    D.dim(inactive ? Math.max(d0, 0.9) : d0);
    frame(L.x0 + i * L.p, L.y, L.z, L.s * k, kind, !inactive);
  }
  D.dim(d0);
}
export function sliceFrameX(name, i) {
  const L = sliceLoc(name);
  return L.x0 + i * L.p;
}

// ---- parts ----
export function drawTape() {
  const bi = T.b;
  // Lid: rises in beat 1, closes again in beat 31.
  let ly = lerp(LID_Y, 7, ph(1, 2.0, 3.5));
  let ls = 1 - rp(1, 3.1, 3.5);
  if (bi >= 31) { ly = lerp(7, LID_Y, ph(31, 2.0, 3.4)); ls = rp(31, 2.0, 2.4); }
  if (bi >= 2 && bi < 31) ls = 0;
  D.dim(0);
  if (ls > 0.001) D.box('ti', 0, ly, 0.9, 27 * ls, 0.2 * ls, 8 * ls, PALETTE.titanium);

  // Everything of the tape side belongs to the 'ribbon' region.
  D.dim(1 - lit('ribbon'));
  // Rack: rail on two posts, three cards, the leaf card lit.
  D.box('ti', 11, 4.2, -5.4, 7, 0.15, 0.15, PALETTE.titanium);
  D.box('ti', 7.7, 2.1, -5.4, 0.15, 4.2, 0.15, PALETTE.titanium);
  D.box('ti', 14.3, 2.1, -5.4, 0.15, 4.2, 0.15, PALETTE.titanium);
  for (let c = 0; c < 3; c++) {
    const x = 11 - 2.2 + c * 2.2;
    const leaf = c === 0;
    D.lbox('gold', x, 3.6, -5.4, 1.6, 1.0, 0.08, PALETTE.gold, PALETTE.goldBright, leaf ? 0.3 : 0);
    for (let n = 0; n < 3 - c; n++) D.box('matte', x - 0.5 + n * 0.4, 3.18, -5.34, 0.18, 0.2, 0.06, PALETTE.ink);
    D.box('ti', x, 4.1, -5.4, 0.04, 0.2, 0.04, PALETTE.titanium);
  }
  // Lane line and ribbon slots.
  D.line(-16, -5.3, 8.5, -5.3, 1, 0.03);
  D.box('matte', 0, 0.025, -5.3, 16, 0.05, 1.2, PALETTE.paperWarm);
  D.box('ti', 0, 0.06, -5.9, 16.1, 0.05, 0.06, PALETTE.titanium);
  D.box('ti', 0, 0.06, -4.7, 16.1, 0.05, 0.06, PALETTE.titanium);
  for (let i = 0; i <= 16; i++) D.box('ti', -8 + i, 0.06, -5.3, 0.05, 0.05, 1.2, PALETTE.titanium);
  // Waiting areas (scale 0.4).
  D.box('matte', -12.4, 0.01, -5.3, 6.6, 0.02, 0.6, PALETTE.paperWarm);
  if (bi >= 1) D.box('matte', -12.4, 0.01, -3.9, 6.6, 0.02, 0.6, PALETTE.paperWarm);

  // Slot ruler with 16 ticks at z -4.5 (from beat 0, 4.6 s).
  const rk = rp(0, 4.6, 5.2);
  if (rk > 0.01) {
    D.box('ti', 0, 0.05, -4.45, 16 * rk, 0.1, 0.12, PALETTE.titanium);
    for (let i = 0; i < 16; i++) if ((i + 0.5) / 16 <= rk) D.box('matte', -7.5 + i, 0.12, -4.45, 0.05, 0.04, 0.22, PALETTE.onSurface);
  }

  // Cutter: blade on two posts, drops and rises at 4.0 to 4.8 of beat 0.
  const drop = ph(0, 4.0, 4.4) - ph(0, 4.4, 4.8);
  D.box('ti', -8.2, 0.9, -6.0, 0.12, 1.8, 0.12, PALETTE.titanium);
  D.box('ti', -8.2, 0.9, -4.6, 0.12, 1.8, 0.12, PALETTE.titanium);
  D.box('ti', -8.2, lerp(1.2, 0.7, drop), -5.3, 0.1, 1.2, 1.5, PALETTE.titanium);

  // Incoming tape from scene 2: eight frames at scale 0.5 on the plate, slides to the lane start.
  const m = ph(0, 2.0, 3.6);
  const ms = 1 - rp(0, 3.2, 3.6);
  if (ms > 0.001) {
    const cx = lerp(-20, -13, m);
    const cz = lerp(0, -5.3, m);
    const cy = lerp(0.2, 0.1, m);
    for (let i = 0; i < 8; i++) frame(cx + (i - 3.5) * 0.5, cy, cz, 0.5 * ms, 'rule', true);
  }
  const k = ph(0, 3.0, 3.6);
  if (k > 0.001) {
    // Tape strip that joins the slices until the blade cuts it.
    if (bi === 0 && T.tb < 4.2) D.box('matte', -7.8, 0.12, -5.3, 2.8 * k, 0.04, 0.3, PALETTE.ceramic);
    drawSlice('A', k);
    drawSlice('T', k);
  }
  if (bi >= 1) {
    drawSlice('G', 1);
    if (bi >= 1) { /* A and T are drawn above from beat 0 on */ }
  }
  if (bi === 3) D.sweepX(-7.5, 1.5, 0.45, -5.3, 1.3, rp(3, 2.0, 4.4), true);
  D.dim(0);
}

// Brackets under the active prefix and inactive suffix (beat 3.4).
export function drawBrackets() {
  if (T.b !== 3 && T.b !== 4) return;
  const k = T.b === 3 ? ph(3, 4.4, 5.4) : 1 - ph(4, 0, 0.6);
  if (k <= 0.001) return;
  D.dim(1 - lit('ribbon'));
  const z = -4.1;
  D.box('ti', -2.5, 0.05, z, 10 * k, 0.04, 0.06, PALETTE.titanium);
  D.box('ti', -7.95, 0.05, z + 0.12, 0.06, 0.04, 0.3, PALETTE.titanium);
  D.box('ti', 2.95, 0.05, z + 0.12, 0.06, 0.04, 0.3, PALETTE.titanium);
  D.box('matte', 5.25, 0.05, z, 5 * k, 0.04, 0.06, PALETTE.onSurface);
  D.box('matte', 2.95 + 0.1, 0.05, z + 0.12, 0.06, 0.04, 0.3, PALETTE.onSurface);
  D.box('matte', 7.95, 0.05, z + 0.12, 0.06, 0.04, 0.3, PALETTE.onSurface);
  D.dim(0);
}
