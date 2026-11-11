// HASH, VALUE DIGEST, and the beat overlays on the memory side (beats 3.10, 3.14 to 3.16, 3.24).
import { PALETTE, FAMILY, lerp } from '../../theme.js';
import * as D from './draw.js';
import { T, ph, rp } from './time.js';
import { lit } from './lit.js';
import { hashPh, eq } from './motion.js';
import { winX, WIN_Z, TREE_Z, ROOT_Y } from './mem.js';
import { READ, REG_Z, regX } from './core.js';
import { pointAt } from './route.js';

const P = [0, 0, 0];
const FILE_X = (n) => -0.7 - 0.38 * n;
const SLOT_X = -0.34;
const HASH_Z = 2.1;

// Hash block hashH with four inputs gathered along -x and an output token.
function hashRun(bi, a, starts, ends, content, size, outFam = 'state') {
  const h = hashPh(bi, a);
  for (let n = 0; n < 4; n++) {
    const from = starts[n];
    const gather = h.gather;
    const sl = Math.max(0, Math.min(1, h.slide * 4 - n));
    const x = lerp(lerp(from[0], FILE_X(n), gather), SLOT_X, sl);
    const z = lerp(lerp(from[1], HASH_Z, gather), HASH_Z, sl);
    const sc = 1 - sl;
    if (T.tb >= a && sc > 0.01 && h.out < 0.01) {
      if (n === 0 && ends === 'domain') D.cell(x, 0.4, z, 0.28, PALETTE.gold, sc);
      else D.cell(x, 0.4, z, 0.28, PALETTE.ceramic, sc);
    }
  }
  const o = h.out;
  if (o > 0.001) D.token(lerp(1.34, 2.94, o), 0.3, HASH_Z, size * (0.2 + 0.8 * o), outFam, content);
  return h;
}

export function drawHash() {
  const bi = T.b;
  D.dim(1 - lit('hash'));
  D.housing(0.5, 2.1, 3, 1.5);
  const gl = bi === 9 ? hashPh(9, 2.0).glow : bi === 23 ? hashPh(23, 3.0).glow : 0;
  D.hashBlock(0.5, 0.3, 2.1, 0.7, gl);
  if (bi === 9) {
    // Three direct lines from CORE, then the hash with a domain cell first.
    const lk = ph(9, 1.2, 2.0);
    const zs = [0.1, 1.0, 1.7];
    for (let n = 0; n < 3; n++) D.line(-4.5, zs[n], FILE_X(n + 1), HASH_Z, lk, 0.5, PALETTE.onSurface, 0.05);
    const starts = [[-0.85, 2.5], [-4.5, zs[0]], [-4.5, zs[1]], [-4.5, zs[2]]];
    hashRun(9, 2.0, starts, 'domain', 'hash:digest', 0.8);
    // Provider tuple with a 3-pip chip cell on the bus branch (6.0 s).
    const tk = ph(9, 6.0, 6.5);
    if (tk > 0.01) D.tuple(-1.6, 0.05, HASH_Z + 0.35, 3, { yaw: 0, s: 0.45, lit: 0, k: tk });
  }
  D.dim(0);
}

// VALUE DIGEST housing and the creation hash (beat 3.24).
export function drawVd() {
  const bi = T.b;
  D.dim(1 - lit('vd'));
  D.housing(4.3, 2.1, 3, 1.5);
  const fk = bi === 23 ? ph(23, 3.0, 3.4) : 0;
  D.lbox('matte', 4.3, 0.34, 2.55, 1.1, 0.05, 0.4, PALETTE.ceramic, PALETTE.goldBright, bi >= 23 ? fk : 0);
  D.dim(0);
  if (bi !== 23) return;
  D.dim(1 - lit('hash'));
  // Four cells travel from the read port along a direct connector to the left of hashH.
  const k = ph(23, 2.0, 3.0);
  const wire = [[-4.9, 0.8], [-4.0, 0.8], [-4.0, HASH_Z], [-1.1, HASH_Z]];
  D.wire(wire, ph(23, 1.2, 2.2), 0.5, PALETTE.onSurface);
  const h = hashPh(23, 3.0);
  const route = [[-4.9, 0.5, 0.8], [-4.0, 0.5, 0.8], [-4.0, 0.5, HASH_Z], [-1.1, 0.5, HASH_Z]];
  if (T.tb >= 2.0 && h.out < 0.01) {
    for (let n = 0; n < 4; n++) {
      const g = Math.min(1, Math.max(0, k * 1.0 - n * 0.0));
      pointAt(route, g, P);
      const sl = Math.max(0, Math.min(1, h.slide * 4 - n));
      const x = lerp(P[0] + (g >= 1 ? -n * 0.38 : 0), SLOT_X, sl);
      const z = lerp(P[2], HASH_Z, sl);
      if (1 - sl > 0.01) D.cell(x, 0.4, z, 0.28, n === 0 ? PALETTE.gold : PALETTE.ceramic, 1 - sl);
    }
  }
  const o = h.out;
  const t2 = ph(23, 6.4, 7.6);
  if (o > 0.001) {
    const x = lerp(lerp(1.34, 2.94, o), winX(6), t2);
    const z = lerp(HASH_Z, WIN_Z, t2);
    const y = lerp(0.3, 0.5, t2);
    const s = lerp(0.8 * (0.2 + 0.8 * o), 0.5, t2);
    D.token(x, y, z, s, 'state', 'vd:new');
  }
  D.dim(0);
}

// Beat 3.15: step counter and capacity rulers; beat 3.16: the tape copy and the cut marker.
export function drawExtras() {
  const bi = T.b;
  if (bi === 14) {
    D.dim(1 - lit('tree'));
    const k = ph(14, 3.0, 3.6);
    if (k > 0.01) {
      const y = 0.36;
      const w = 8 * k;
      D.box('ti', 9.5, y, -1.2, w, 0.1, 0.25, PALETTE.titanium);
      D.box('ti', 9.5, y, -0.6, w, 0.1, 0.25, PALETTE.titanium);
      // Counter ruler ends at three quarters; capacity ruler runs the full length; break marks on both.
      D.box('matte', 5.5 + 6 * k, y + 0.07, -1.2, 0.06, 0.05, 0.3, PALETTE.onSurface);
      D.box('matte', 5.5 + 8 * k, y + 0.07, -0.6, 0.06, 0.05, 0.3, PALETTE.onSurface);
      D.breakMark(9.0, y + 0.07, -1.2, 0.3);
      D.breakMark(9.0, y + 0.07, -0.6, 0.3);
      // Third bar: three times the counter mark, ends between the two end ticks.
      D.box('gold', 5.5 + 3.4, y + 0.1, -0.9, 6.8, 0.06, 0.1, PALETTE.gold);
      D.sweepX(5.4, 13.6, y + 0.18, -0.9, 1.0, rp(14, 4.0, 5.2));
    }
    D.dim(0);
  }
  if (bi === 14) {
    // Address cell beside the root with a gold outline.
    D.dim(1 - lit('tree'));
    D.push(10.9, ROOT_Y, TREE_Z, 0, 0, 0, 1);
    D.cell(0, -0.2, 0, 0.4);
    D.pop();
    D.dim(0);
  }
  if (bi === 15) {
    D.dim(1 - lit('mem'));
    const mv = ph(15, 2.0, 3.2);
    const mx = lerp(5.5, 5.0, mv);
    for (let i = 0; i < 8; i++) {
      const w = i === 6;
      const shift = w ? 0.4 * ph(15, 3.4, 4.2) : 0;
      const x = 2.25 + 0.5 * i + shift;
      D.box('matte', x, 0.4, 2.6, 0.45, 0.06, 0.6, PALETTE.ceramic);
      D.box('matte', x, 0.45, 2.6, 0.15, 0.04, 0.3, PALETTE.heat);
      if (w && mv > 0.99) D.outline(x, 0.5, 2.6, 0.5, 0.65, PALETTE.heat, 0.05);
    }
    D.box('ti', mx, 0.7, 2.6, 0.1, 0.6, 0.8, PALETTE.titanium);
    D.token(mx - 0.9, 0.3, 3.15, 0.6, 'state', 'state:cut');
    D.token(mx + 0.9, 0.3, 3.15, 0.6, 'state', 'state:cut');
    D.dim(0);
  }
  if (bi === 16) {
    D.dim(1 - lit('core'));
    const a = ph(16, 2.0, 2.6);
    for (let i = 0; i < 3; i++) D.fcell(READ[i][0], 0.3, READ[i][1], 0.46, 'step', a * 0.8, PALETTE.ceramic);
    D.line(READ[0][0], READ[0][1], regX(7), REG_Z[1], a, 0.62, PALETTE.onSurface, 0.04);
    D.line(READ[2][0], READ[2][1], -8.0, 1.9, a, 0.62, PALETTE.onSurface, 0.04);
    D.sweepZ(0.1, 1.5, 0.55, -4.9, 0.6, rp(16, 3.0, 3.8));
    D.dim(1 - lit('mem'));
    const g = ph(16, 4.0, 4.4);
    // Permitted tile (gold outline), a marker line across the tray, two later tiles in step colour.
    D.outline(winX(3), 0.55, WIN_Z, 0.5, 0.5, PALETTE.gold, 0.05, g > 0.5 ? 0.3 : 0);
    D.box('ti', (winX(4) + winX(5)) / 2, 0.5, WIN_Z, 0.05, 0.1, 1.0, PALETTE.titanium);
    D.dim(0);
    if (g > 0.5) { D.outline(winX(5), 0.55, WIN_Z, 0.5, 0.5, PALETTE.heat, 0.06); D.outline(winX(6), 0.55, WIN_Z, 0.5, 0.5, PALETTE.heat, 0.06); }
  }
}
