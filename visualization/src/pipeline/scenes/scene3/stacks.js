// STACKS, INIT and BOUNDARY COMMITMENT (beats 3.21 to 3.23, 3.26 to 3.30).
import { PALETTE, FAMILY, lerp } from '../../theme.js';
import * as D from './draw.js';
import { T, ph, rp } from './time.js';
import { lit } from './lit.js';
import { hashPh, eq } from './motion.js';
import { pointAt } from './route.js';
import { regX, REG_Z, CHAIN_X, CHAIN_Z } from './core.js';
import { winX, tailX } from './mem.js';

const P = [0, 0, 0];
export const FR_X = -7.6;
export const TK_X = -5.0;
export const ST_Z = 4.3;
export const HS = { x: -6.3, z: 4.3 };
const linkY = (j) => 1.05 + 0.7 * j;
const SLOT_IN = [-7.02, 0.7, 4.3];

// Input travels from `from` [x, y, z] to the slot over the hash motion's gather and slide phases.
function inputAt(h, n, count, from, slot, out) {
  const sl = Math.max(0, Math.min(1, h.slide * count - n));
  const g = h.gather * 0.5 + 0.5 * sl;
  out[0] = lerp(from[0], slot[0], g);
  out[1] = lerp(from[1], slot[1], g);
  out[2] = lerp(from[2], slot[2], g);
  return 1 - (sl > 0.999 ? 1 : sl);
}
const IP = [0, 0, 0];

function chain(x, links, notches, shelfCell, shelfLit, dimK) {
  D.cell(x, 0.3, ST_Z, 0.4, PALETTE.gold);
  for (let n = 0; n < notches; n++) D.box('matte', x - 0.1 + n * 0.2, 0.72, ST_Z + 0.15, 0.08, 0.02, 0.1, PALETTE.ink);
  for (let j = 0; j < links; j++) {
    D.token(x, linkY(j), ST_Z, 0.6, 'state', `link:${x < -6 ? 'f' : 't'}${j}`, { lay: false });
    if (j > 0) D.vbar(x, ST_Z, linkY(j - 1) + 0.3, linkY(j) - 0.3, PALETTE.gold, 0.1);
  }
  const top = linkY(Math.max(links - 1, 0)) + 0.55;
  D.box('ti', x, top, ST_Z, 0.7, 0.05, 0.7, PALETTE.titanium);
  return top;
}

// STACKS housing with FRAMES and TASKS chains and the hash block hashS.
export function drawStacks() {
  const bi = T.b;
  D.dim(1 - lit('stacks'));
  D.housing(-6.3, ST_Z, 4, 1.8);
  const gl = bi === 20 ? hashPh(20, 2.0).glow : bi === 21 ? hashPh(21, 2.4).glow : bi === 22 ? hashPh(22, 2.0).glow : bi === 29 ? hashPh(29, 2.4).glow : 0;
  D.hashBlock(HS.x, 0.3, HS.z, 0.6, gl);
  const genesis = bi >= 28 && bi <= 28;
  let fl = 2;
  let tl = 1;
  let fTop = 0;
  if (bi === 20) fl = T.tb >= 5.0 ? 3 : 2;
  if (bi >= 21) fl = 3;
  if (bi === 21 && T.tb >= 7.4) fl = 2;
  if (bi > 21) fl = 2;
  if (bi === 22 && T.tb >= 5.0) tl = 2;
  if (bi > 22) tl = 2;
  if (bi >= 28 && bi <= 28) { fl = 0; tl = 0; }
  if (bi === 29) { fl = 0; tl = 0; }
  const frDim = bi === 22 ? 0.6 : 0;
  const tkDim = bi === 20 || bi === 21 ? 0.6 : 0;
  // Rising animations: output token climbs onto FRAMES (beat 3.21).
  D.dim(Math.max(1 - lit('stacks'), frDim));
  const top1 = chain(FR_X, fl, 2, 0, 0, 0);
  D.dim(Math.max(1 - lit('stacks'), tkDim));
  const top2 = chain(TK_X, tl, 1, 0, 0, 0);
  D.dim(1 - lit('stacks'));
  // Cached top cells on the shelves.
  const frTop = linkY(Math.max(fl - 1, 0)) + 0.55;
  const tkTop = linkY(Math.max(tl - 1, 0)) + 0.55;
  if (bi === 20) {
    const old = T.tb < 5.0 ? 1 : 0;
    if (old) D.cell(FR_X, linkY(1) + 0.58, ST_Z, 0.35, PALETTE.ceramic);
    const drop = ph(20, 5.4, 6.2);
    if (T.tb >= 5.4) D.cell(FR_X, lerp(frTop + 0.9, frTop + 0.03, drop), ST_Z, 0.35, PALETTE.heat);
    D.cell(TK_X, tkTop + 0.03, ST_Z, 0.35, PALETTE.ceramic);
  } else if (bi === 21) {
    const rest = ph(21, 7.4, 8.2);
    const prev = T.tb < 7.4 ? frTop : lerp(frTop, linkY(1) + 0.55, 1);
    D.cell(FR_X, T.tb < 7.4 ? frTop + 0.03 : linkY(1) + 0.58, ST_Z, 0.35, PALETTE.heat);
    D.cell(TK_X, tkTop + 0.03, ST_Z, 0.35, PALETTE.ceramic);
  } else if (bi >= 22 && bi < 28 || bi >= 30) {
    D.cell(FR_X, linkY(1) + 0.58, ST_Z, 0.35, PALETTE.ceramic);
    if (bi === 22) {
      const drop = ph(22, 5.4, 6.2);
      D.cell(TK_X, T.tb < 5.0 ? linkY(0) + 0.58 : lerp(tkTop + 0.9, tkTop + 0.03, drop), ST_Z, 0.35, T.tb < 5.0 ? PALETTE.heat : PALETTE.ceramic);
      // Superseded cursor lies dimmed with an ink stroke across it.
      D.dim(0.8);
      D.cell(TK_X + 0.9, 0.3, ST_Z + 0.5, 0.3, PALETTE.ceramic);
      D.box('matte', TK_X + 0.9, 0.62, ST_Z + 0.5, 0.4, 0.02, 0.05, PALETTE.ink, 0.6);
      D.dim(1 - lit('stacks'));
    } else D.cell(TK_X, tkTop + 0.03, ST_Z, 0.35, PALETTE.ceramic);
  } else if (bi < 20) {
    D.cell(FR_X, frTop + 0.03, ST_Z, 0.35, PALETTE.ceramic);
    D.cell(TK_X, tkTop + 0.03, ST_Z, 0.35, PALETTE.ceramic);
  }

  // Hash motions of STACKS.
  if (bi === 20) {
    const h = hashPh(20, 2.0);
    const from = [[FR_X, linkY(1) + 0.6, ST_Z], [FR_X - 0.5, 0.5, ST_Z + 0.9], [FR_X, linkY(1), ST_Z]];
    for (let n = 0; n < 3; n++) {
      if (T.tb < 2.0 || h.out > 0.01) break;
      const k = inputAt(h, n, 3, from[n], SLOT_IN, IP);
      if (k > 0.01) D.cell(IP[0], IP[1], IP[2], 0.28 * k, n === 2 ? PALETTE.gold : PALETTE.ceramic);
    }
    const o = h.out;
    const r = ph(20, 5.0, 5.4);
    if (o > 0.001) D.token(lerp(lerp(-5.58, -5.58, o), FR_X, r), lerp(0.7, linkY(2), r), ST_Z, 0.6 * (0.2 + 0.8 * o), 'state', 'link:f2', { lay: false });
  }
  if (bi === 21) {
    const a = ph(21, 2.0, 2.4);
    const h = hashPh(21, 2.4);
    if (a > 0.001 && T.tb < 7.4) {
      const nodes = [[FR_X - 0.6, 0.35, ST_Z + 1.0], [FR_X + 0.0, 0.35, ST_Z + 1.2], [FR_X + 0.6, 0.35, ST_Z + 1.0]];
      for (let n = 0; n < 3; n++) {
        if (h.out > 0.01) break;
        const k = T.tb < 2.4 ? 1 : inputAt(h, n, 3, nodes[n], SLOT_IN, IP);
        const pos = T.tb < 2.4 ? nodes[n] : IP;
        if (k > 0.01) {
          if (n === 2) D.token(pos[0], pos[1] + 0.2, pos[2], 0.4 * a * k, 'state', 'link:f1', { lay: false });
          else D.cell(pos[0], pos[1], pos[2], 0.28 * a * k, n === 0 ? PALETTE.heat : PALETTE.ceramic);
        }
      }
    }
    const o = h.out;
    const e = eq(21, 5.8);
    if (o > 0.001 && T.tb < 7.4 && e.show) D.token(lerp(lerp(-5.58, FR_X + 0.6, o), FR_X, e.m), lerp(0.7, linkY(2), o), ST_Z, 0.6 * (0.2 + 0.8 * o), 'state', 'link:f2', { lay: false });
    if (e.ring && T.tb < 7.4) D.ring(FR_X, linkY(2), ST_Z, 0.7, 0.7, e.lit);
    // The top link lowers out of the chain and hides.
    if (T.tb < 7.4) D.token(FR_X, linkY(2), ST_Z, 0.6, 'state', 'link:f2', { lay: false });
    else {
      const lw = 1 - ph(21, 7.4, 8.2);
      if (lw > 0.01) D.token(FR_X, linkY(2) - 0.8 * (1 - lw), ST_Z, 0.6 * lw, 'state', 'link:f2', { lay: false });
    }
  }
  if (bi === 22) {
    const h = hashPh(22, 2.0);
    const from = [[TK_X, linkY(0) + 0.55, ST_Z], [TK_X + 0.5, 0.4, ST_Z + 0.9], [TK_X, linkY(0), ST_Z]];
    for (let n = 0; n < 3; n++) {
      if (T.tb < 2.0 || h.out > 0.01) break;
      const k = inputAt(h, n, 3, from[n], [-5.58, 0.7, 4.3], IP);
      if (k > 0.01) D.cell(IP[0], IP[1], IP[2], 0.28 * k, n === 0 ? PALETTE.heat : PALETTE.ceramic);
    }
    const o = h.out;
    const r = ph(22, 5.0, 5.4);
    if (o > 0.001) D.token(lerp(-5.58 + 0.0, TK_X, r), lerp(0.7, linkY(1), r), ST_Z, 0.6 * (0.2 + 0.8 * o), 'state', 'link:t1', { lay: false });
    // Tape strip of three frames at scale 0.5 in front of the housing at z 5.5, tied to both changes.
    for (let i = 0; i < 3; i++) {
      const x = -6.8 + i * 0.5;
      D.box('matte', x, 0.1, 5.5, 0.45, 0.06, 0.6, PALETTE.ceramic);
      D.box('matte', x, 0.15, 5.5, 0.15, 0.04, 0.25, PALETTE.heat);
    }
    const ck = ph(22, 6.2, 6.8);
    D.line(TK_X, ST_Z, -6.3, 5.5, ck, 0.6, PALETTE.onSurface, 0.04);
    D.line(TK_X + 0.9, ST_Z + 0.5, -6.3, 5.5, ck, 0.6, PALETTE.onSurface, 0.04);
  }
  D.dim(0);
}

// INIT housing (beats 3.29, 3.30).
const INIT_X = -10.2;
export function drawInit() {
  const bi = T.b;
  D.dim(1 - lit('init'));
  D.housing(INIT_X, 4.2, 2.6, 1.5);
  const sel = bi >= 28 ? ph(28, 1.0, 1.6) : 0;
  const sweep = (a) => bi === 28 ? rp(28, 2.0 + a, 2.0 + a + 0.5) : 0;
  const lg = (k) => (bi === 28 && T.tb >= 2.0 + k && T.tb < 2.0 + k + 0.7 ? 0.9 : 0);
  // Height cell with an 8-cube collar at the left edge, selector cell lit gold.
  D.cell(-12.0, 0.3, 4.2, 0.4);
  for (let i = 0; i < 8; i++) {
    const a = (i / 8) * Math.PI * 2;
    D.cell(-12.0 + 0.4 * Math.cos(a), 0.3, 4.2 + 0.4 * Math.sin(a), 0.1, PALETTE.ceramicEdge);
  }
  D.line(-11.8, 4.2, -10.9, 4.2, ph(28, 0.4, 1.2) * (bi >= 28 ? 1 : 0), 0.33, PALETTE.onSurface, 0.05);
  D.lbox('gold', -10.8, 0.45, 4.25, 0.35, 0.3, 0.35, PALETTE.gold, PALETTE.goldBright, bi >= 28 ? sel * 0.8 : 0);
  // Outcome cell and three fuel cells.
  D.fcell(-11.0, 0.3, 3.8, 0.3, 'step', lg(0.5), PALETTE.ceramic);
  for (let i = 0; i < 3; i++) D.fcell(-10.4 + 0.45 * i, 0.3, 3.8, 0.3, 'step', lg(1.0), PALETTE.ceramic);
  // Empty tray at scale 0.35.
  D.box('ti', INIT_X, 0.33, 4.75, 2.6, 0.05, 0.4, PALETTE.titanium);
  for (let i = 0; i < 8; i++) D.outline(INIT_X + (i - 3.5) * 0.31, 0.37, 4.75, 0.26, 0.26, PALETTE.onSurface, 0.02);
  if (bi === 28) {
    // Sweep order: regs before row, outcome cell, fuel cells, STACKS bases, empty tray.
    for (let i = 0; i < 8; i++) D.fcell(regX(i), 0.3, REG_Z[0], 0.46, 'step', lg(0.0) * 0.7, PALETTE.ceramic);
  }
  D.dim(0);
}

// Beat 3.29 overlays (state token and connectors) and beat 3.30 (input by digest).
export function drawGenesis() {
  const bi = T.b;
  if (bi === 28) {
    D.dim(1 - lit('init'));
    const lg = (k) => (T.tb >= 2.0 + k && T.tb < 2.0 + k + 0.7 ? 0.9 : 0);
    D.fcell(FR_X, 0.3, ST_Z, 0.5, 'step', lg(1.5), PALETTE.gold);
    D.fcell(TK_X, 0.3, ST_Z, 0.5, 'step', lg(1.5), PALETTE.gold);
    const k = ph(28, 5.0, 5.6);
    if (k > 0.001) {
      D.token(-9.5, 0.3, 4.2, 0.6 * (0.2 + 0.8 * k), 'state', 'state:init', { ring: k });
      D.token(-8.6, 0.3, 3.9, 0.35, 'program', 'prog:main', { k });
      D.token(-8.6, 0.3, 4.5, 0.35, 'input', 'input:main', { k });
      D.line(-9.5, 4.2, -10.4, 3.8, k, 0.5, PALETTE.onSurface, 0.03);
      D.line(-9.5, 4.2, INIT_X, 4.75, k, 0.5, PALETTE.onSurface, 0.03);
      D.line(-9.5, 4.2, -8.6, 3.9, k, 0.5, PALETTE.onSurface, 0.03);
      D.line(-9.5, 4.2, -8.6, 4.5, k, 0.5, PALETTE.onSurface, 0.03);
      D.line(-9.5, 4.2, FR_X, ST_Z, k, 0.5, PALETTE.onSurface, 0.03);
      D.line(-10.4, 3.8, regX(0), REG_Z[0], k, 0.5, PALETTE.onSurface, 0.03);
    }
    D.dim(0);
  }
  if (bi === 29) {
    D.dim(1 - lit('init'));
    D.token(-9.5, 0.3, 4.2, 0.6, 'state', 'state:init', { ring: 1 });
    D.token(-8.6, 0.3, 3.9, 0.35, 'program', 'prog:main');
    D.token(-8.6, 0.3, 4.5, 0.35, 'input', 'input:main');
    D.dim(1 - lit('core'));
    // Reference token in a regs cell position.
    const e = eq(29, 5.8);
    D.token(regX(3), 0.52, REG_Z[1], 0.5, 'input', 'val:input', { k: 1 });
    D.dim(1 - lit('init'));
    // Input node: four cells in front of INIT.
    const nk = ph(29, 2.0, 2.4);
    const h = hashPh(29, 2.4);
    const move = ph(29, 7.4, 8.4);
    if (nk > 0.001 && T.tb < 7.4) {
      for (let n = 0; n < 4; n++) {
        const from = [-9.0 + 0.45 * n, 0.4, 5.4];
        let x = from[0]; let y = from[1]; let z = from[2]; let k = nk;
        if (T.tb >= 2.4) k = inputAt(h, n, 4, from, SLOT_IN, IP) * nk;
        if (T.tb >= 2.4) { x = IP[0]; y = IP[1]; z = IP[2]; }
        if (h.out < 0.01 && k > 0.01) D.cell(x, y, z, 0.28 * k, PALETTE.ceramic);
      }
    }
    if (T.tb >= 7.4 || h.out > 0.99) {
      // Node cells travel to the first chain event cube.
      for (let n = 0; n < 4; n++) {
        const q = T.tb >= 7.4 ? move : 0;
        D.cell(lerp(-9.0 + 0.45 * n, CHAIN_X[1] + (n - 1.5) * 0.18, q), 0.4, lerp(5.4, CHAIN_Z, q), 0.28 * (T.tb >= 7.4 ? 1 : 0));
      }
    }
    const o = h.out;
    if (o > 0.001 && T.tb < 7.4 && e.show) {
      const px = lerp(lerp(-5.58, -4.6, o), regX(3), e.m);
      const pz = lerp(4.3, REG_Z[1], e.m);
      D.token(px, 0.55, pz, 0.6 * (0.2 + 0.8 * o), 'input', 'val:input');
    }
    if (e.ring) D.ring(regX(3), 0.55, REG_Z[1], 0.55, 0.55, e.lit);
    D.dim(0);
  }
}

// BOUNDARY COMMITMENT (beats 3.26 to 3.28).
export function drawBoundary() {
  const bi = T.b;
  D.dim(1 - lit('bnd'));
  D.housing(9.5, 4.2, 7.8, 2);
  const g1 = bi === 25 ? hashPh(25, 3.4).glow : 0;
  const g2 = bi === 26 ? hashPh(26, 3.0).glow : 0;
  D.hashBlock(7.6, 0.3, 4.2, 0.7, g1);
  D.hashBlock(11.4, 0.3, 4.2, 0.7, g2);
  D.dim(0);
  if (bi < 25 || bi > 27) return;
  D.dim(1 - lit('bnd'));
  if (bi === 25) {
    const k = ph(25, 2.0, 3.4);
    const h = hashPh(25, 3.4);
    const paths = [
      [[-4.5, -1.0], [-4.0, -1.0], [-4.0, 5.35], [5.9, 5.35], [5.9, 4.4], [6.4, 4.4]],
      [[-4.3, 5.1], [-4.0, 5.1], [-4.0, 5.2], [5.7, 5.2], [5.7, 4.2], [6.4, 4.2]],
      [[6.4, 0.5], [5.7, 0.5], [5.7, 4.0], [6.4, 4.0]],
    ];
    const lk = ph(25, 1.4, 2.4);
    for (const p of paths) D.wire(p, bi === 25 ? Math.max(lk, 0) : 1, 0.5, PALETTE.onSurface);
    const sizes = [0.2, 0.4, 0.36];
    for (let g = 0; g < 3; g++) {
      const f = Math.max(0, Math.min(1, h.slide * 3 - g));
      const along = k * (h.slide > 0 ? 1 : 1);
      let x; let z;
      const p = paths[g];
      const q = [p[0][0], 0.5, p[0][1]];
      // Position along the connector polyline.
      const route = p.map((pt) => [pt[0], 0.5, pt[1]]);
      pointAt(route, along, P);
      const end = route[route.length - 1];
      x = lerp(P[0], 6.4 + 0.0, f); z = lerp(P[2], end[2], f);
      const sc = 1 - f;
      if (T.tb >= 2.0 && sc > 0.01 && h.out < 0.01) {
        if (g === 0) for (let i = 0; i < 4; i++) D.cell(x - i * 0.2, 0.5, z, sizes[0] * sc);
        else if (g === 1) { D.token(x, 0.5, z, sizes[1] * sc, 'state', 'link:f0'); D.token(x - 0.45, 0.5, z, sizes[1] * sc, 'state', 'link:t0'); }
        else { D.cell(x, 0.5, z, 0.25 * sc); D.token(x - 0.4, 0.5, z, sizes[2] * sc, 'state', 'root:post'); D.token(x - 0.85, 0.5, z, sizes[2] * sc, 'state', 'tail:pre'); }
      }
    }
    const o = h.out;
    if (o > 0.001) D.token(lerp(8.4, 9.5, o), 0.3, 4.2, 0.8 * (0.2 + 0.8 * o), 'state', 'body:A');
  }
  if (bi >= 26) D.token(9.5, 0.3, 4.2, 0.8, 'state', 'body:A');
  if (bi === 26) {
    const a = ph(26, 2.0, 2.8);
    const h = hashPh(26, 3.0);
    const hx = [6.6, 7.5, 8.4, 9.3];
    if (a > 0.001) {
      for (let n = 0; n < 4; n++) {
        const sl = Math.max(0, Math.min(1, h.slide * 5 - n));
        const gx = lerp(lerp(hx[n], 10.0 - n * 0.3, h.gather), 10.2, sl);
        const gz = lerp(lerp(3.2, 4.2, h.gather), 4.2, sl);
        const x = T.tb < 3.0 ? hx[n] : gx; const z = T.tb < 3.0 ? 3.2 : gz;
        // Copies travel while the five originals stay readable.
        if (n < 2) D.cell(hx[n], 0.3, 3.2, 0.3, n === 0 ? PALETTE.resolve : PALETTE.ceramic, a);
        else D.token(hx[n], 0.3, 3.2, 0.5, n === 2 ? 'input' : 'state', n === 2 ? 'input:main' : 'deps:main', { k: a });
        if (T.tb >= 3.0 && (1 - sl) > 0.01 && h.out < 0.01) {
          if (n < 2) D.cell(x, 0.5, z, 0.25 * (1 - sl));
          else D.token(x, 0.5, z, 0.4 * (1 - sl), n === 2 ? 'input' : 'state', n === 2 ? 'input:main' : 'deps:main');
        }
      }
      const sl5 = Math.max(0, Math.min(1, h.slide * 5 - 4));
      if (T.tb >= 3.0 && (1 - sl5) > 0.01 && h.out < 0.01) D.token(lerp(lerp(9.5, 9.3, h.gather), 10.2, sl5), 0.5, lerp(4.2, 4.2, sl5), 0.5 * (1 - sl5), 'state', 'body:A');
    }
    const o = h.out;
    if (o > 0.001) D.token(lerp(12.2, 13.2, o), 0.3, 4.2, 0.8 * (0.2 + 0.8 * o), 'state', 'state:A', { ring: o });
  }
  if (bi === 27) {
    D.token(13.2, 0.3, 4.2, 0.8, 'state', 'state:A', { ring: 1 });
    D.token(6.2, 0.3, 5.4, 0.8, 'state', 'state:pre', { ring: 1 });
    // Pin sockets (hexagonal collars) holding public tokens of the same content.
    D.push(6.2, 0.4, 3.0, 0, 0, 0, 1); D.pop();
    const pins = [[6.2, 3.0, 'state:pre'], [13.2, 3.0, 'state:A']];
    for (let i = 0; i < 2; i++) {
      D.token(pins[i][0], 0.35, pins[i][1], 0.36, 'state', pins[i][2], { ring: 1 });
      const e = i === 0 ? eq(27, 2.0) : eq(27, 3.6);
      const sx = pins[i][0];
      const sz = i === 0 ? 5.4 : 4.2;
      if (T.tb >= (i === 0 ? 2.0 : 3.6) && e.show) D.token(sx, 0.55, lerp(sz, 3.0, e.m), 0.8 * 0.8, 'state', pins[i][2], { ring: 1 });
      if (e.ring) D.ring(pins[i][0], 0.5, pins[i][1], 0.4, 0.4, e.lit);
    }
    // Scratch pad with three cells joined by two short arrows inside one slot outline.
    const pk = ph(27, 6.0, 6.6);
    if (pk > 0.001) {
      D.box('matte', 9.5, 0.05 * pk, 5.6, 2.8 * pk, 0.1 * pk, 0.8 * pk, PALETTE.paperWarm);
      D.outline(9.5, 0.12, 5.6, 2.6, 0.7, PALETTE.onSurface, 0.04);
      for (let i = 0; i < 3; i++) D.cell(8.8 + 0.7 * i, 0.12, 5.6, 0.3, PALETTE.ceramic, pk);
      D.line(9.0, 5.6, 9.35, 5.6, pk, 0.3, PALETTE.onSurface, 0.04);
      D.line(9.7, 5.6, 10.05, 5.6, pk, 0.3, PALETTE.onSurface, 0.04);
    }
  }
  D.dim(0);
}
