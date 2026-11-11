// MEM window, MEMORY BACKEND, segments, the memory tree, PAGE/ROM, the row budget (beats 3.11 to 3.20).
import { PALETTE, FAMILY, lerp } from '../../theme.js';
import * as D from './draw.js';
import { T, ph, rp, after, win, START, treeK } from './time.js';
import { lit } from './lit.js';
import { eq, hashPh, lerp3 } from './motion.js';
import { pointAt, routeYaw, polyLen } from './route.js';
import { CHAIN_X, CHAIN_Z, READ } from './core.js';
import { tupleOn } from './bus.js';

export const winX = (i) => 1.5 + (i - 3.5) * 0.54;
export const WIN_Z = 0.2;
export const tailX = (i) => 9.5 + (i - 3.5) * 0.9;
export const TAIL_Z = 0.9;
export const TREE_X = 9.5;
export const TREE_Z = -3.6;
export const NODE_Y = [1.0, 1.7, 2.4, 3.8, 4.5];
export const ROOT_Y = 5.4;
const P = [0, 0, 0];
const CAPS = 0.6;

// Window tiles: 0 to 2 imported (titanium outline), 3 to 5 new, 6 new after beat 3.12, 7 padding.
function winTile(i, k, o = {}) {
  D.tile(winX(i), 0.3, WIN_Z, CAPS * k, o.hex ?? PALETTE.ceramic, true, o.lit ?? 0);
}
export const newTilesOn = () => (T.b >= 10 ? 3 : 0);

export function drawMem() {
  const bi = T.b;
  D.dim(1 - lit('mem'));
  D.housing(1.5, 0.2, 5, 1.5);
  // Tray of the window at scale 0.6.
  D.box('ti', 1.5, 0.33, WIN_Z, 4.5, 0.06, 0.7, PALETTE.titanium);
  for (let i = 0; i < 8; i++) {
    const x = winX(i);
    if (i < 3) { winTile(i, 1); D.outline(x, 0.5, WIN_Z, 0.5, 0.5, PALETTE.titanium, 0.04); }
    else if (i < 6) {
      const landed = bi > 10 || (bi === 10 && T.tb > 3.3 + (i - 3) * 1.0 + 0.0);
      if (bi >= 11) winTile(i, 1);
      else if (bi === 10) D.outline(x, 0.4, WIN_Z, 0.5, 0.5, PALETTE.onSurface, 0.03);
      else D.outline(x, 0.4, WIN_Z, 0.5, 0.5, PALETTE.onSurface, 0.03);
    } else if (i === 6) {
      const k = bi === 11 ? ph(11, 2.0, 3.0) : bi > 11 ? 1 : 0;
      if (k < 0.99) D.box('matte', x, 0.4, WIN_Z, 0.48, 0.06, 0.48, PALETTE.paperWarm);
      if (k > 0.01) winTile(i, 1, {});
    } else D.box('matte', x, 0.4, WIN_Z, 0.48, 0.06, 0.48, PALETTE.paperWarm);
  }
  D.dim(0);

  // Beat 3.11: three tuples from the append chain land on the window.
  if (bi === 10) {
    D.dim(1 - lit('mem'));
    for (let j = 0; j < 3; j++) {
      const a = 2.5 + j * 1.0;
      const k = ph(10, a, a + 0.9);
      const f = rp(10, a, a + 0.9);
      if (T.tb < a) continue;
      // Route: event cube, along z 1.9 to lane 0, down the lane to the MEM branch, along it to the tile.
      const x0 = CHAIN_X[j + 1];
      const tx = winX(3 + j);
      const route = [[x0, 0.4, CHAIN_Z], [-3.3, 0.1, CHAIN_Z], [-3.3, 0.1, WIN_Z], [tx, 0.4, WIN_Z]];
      tupleOn(route, k, 2, { s: 0.45 });
      if (k >= 1) { /* rests on its tile */ }
      // The connector from the event cube to its tile stays.
      D.line(x0, CHAIN_Z, tx, WIN_Z, k, 0.62, PALETTE.onSurface, 0.04);
    }
    D.dim(0);
  }
  // Beat 3.12: a fourth tile slides from the MEM branch into the next free position; length markers.
  if (bi === 11 || bi === 12) {
    D.dim(1 - lit('mem'));
    if (bi === 11 && T.tb < 3.0) {
      const k = ph(11, 2.0, 3.0);
      D.tile(lerp(-1.0, winX(6), k), 0.3, WIN_Z, CAPS, PALETTE.ceramic, true);
    }
    const mk = ph(11, 3.4, 4.0);
    if (bi === 12 || mk > 0.01) {
      D.cell(winX(2) + 0.27, 0.0, WIN_Z + 0.55, 0.22);
      D.cell(winX(6) + 0.27, 0.0, WIN_Z + 0.55, 0.22);
      D.line(CHAIN_X[0], CHAIN_Z, winX(2) + 0.27, WIN_Z + 0.55, 1, 0.62, PALETTE.onSurface, 0.03);
      D.line(CHAIN_X[4], CHAIN_Z, winX(6) + 0.27, WIN_Z + 0.55, 1, 0.62, PALETTE.onSurface, 0.03);
    }
    D.dim(0);
  }
}

// How many tiles the tail tray holds: 8 after beat 3.13, empty after the flush, 6 from beat 3.16.
function tailCount() {
  const bi = T.b;
  if (bi < 12) return 0;
  if (bi === 12) return 8;
  if (bi === 13) return 8;
  if (bi === 14) return 0;
  return 6;
}
// Segment token position helper for beat 3.14.
export const SOCKET = [TREE_X - 0.8, NODE_Y[0], TREE_Z];

export function drawBackend() {
  const bi = T.b;
  D.dim(1 - lit('backend'));
  D.housing(9.5, 0.4, 7.8, 2.2);
  // Tail tray (full scale): rim and eight positions.
  D.box('ti', 9.5, 0.38, TAIL_Z, 7.4, 0.06, 1.1, PALETTE.titanium);
  let n = tailCount();
  let tk = 1;
  if (bi === 12) tk = ph(12, 2.0, 3.4);
  if (bi === 13) tk = 1 - ph(13, 2.0, 3.0);
  if (bi === 13 && T.tb >= 3.0) n = 0;
  for (let i = 0; i < 8; i++) {
    const x = tailX(i);
    if (i < n && tk > 0.001 && !(bi === 12 && T.tb < 2.0)) {
      const g = bi === 12 ? ph(12, 2.0, 3.4) : 1;
      const sx = lerp(winX(i), x, g);
      const sz = lerp(WIN_Z, TAIL_Z, g);
      const sc = lerp(CAPS, 1, g);
      const tile = i < 3 ? PALETTE.ceramic : PALETTE.ceramic;
      if (bi === 13) D.tile(lerp(x, TREE_X - 0.8 + 0.0, ph(13, 2.0, 3.0)), 0.4, lerp(TAIL_Z, TREE_Z + 0.9, ph(13, 2.0, 3.0)), 1 - 0.6 * ph(13, 2.0, 3.0), tile);
      else D.tile(sx, bi === 12 ? lerp(0.3, 0.4, g) : 0.4, sz, sc, tile);
      if (i < 3 && bi < 14 && bi !== 13) D.outline(sx, 0.55, sz, 0.8 * sc, 0.8 * sc, PALETTE.titanium, 0.05);
    }
  }
  // Beat 3.13: padding cells flank the hash; positions tail 3 to 7 hold padding in the hash input.
  // Tokens on the backend: len cell, root and tail_d.
  const rootC = bi >= 13 && (bi > 13 || T.tb > 5.4) ? 'root:post' : 'root:pre';
  D.cell(6.4, 0.3, -0.3, 0.4);
  D.token(12.6, 0.3, -0.3, 0.8, 'state', rootC);
  D.token(11.2, 0.3, -0.3, 0.8, 'state', 'tail:pre');
  // Flush unit bracket (beat 3.16): titanium bracket 1.0 wide at the right end, gold fill when used.
  D.box('ti', 12.9, 0.4, 0.4, 1.0, 0.15, 0.1, PALETTE.titanium);
  D.box('ti', 12.9, 0.4, 1.3, 1.0, 0.15, 0.1, PALETTE.titanium);
  if (bi >= 15) D.box('gold', 12.9, 0.35, 0.85, 0.9, 0.1, 0.7, PALETTE.gold);
  // Beat 3.13: hash block at (6.9, 2.1) hashes the tail; output token meets tail_d.
  if (bi === 12) {
    const h = hashPh(12, 4.0);
    const k = ph(12, 3.8, 4.2);
    if (k > 0.001) D.hashBlock(6.9, 0.3, 2.1, 0.7 * k, h.glow);
    if (T.tb > 4.0) {
      // Inputs: three imported tiles and five padding cells, copies of the tail cells.
      for (let i = 0; i < 8; i++) {
        const sl = Math.max(0, Math.min(1, h.slide * 8 - i));
        const g = h.gather;
        const x = lerp(lerp(tailX(i), 4.8 - i * 0.12, g), 5.9, sl);
        const z = lerp(lerp(TAIL_Z, 2.1, g), 2.1, sl);
        const sc = 1 - sl;
        if (h.slide < 1) {
          if (i < 3) D.tile(x, 0.5, z, 0.45 * sc, PALETTE.ceramic, true);
          else D.cell(x, 0.5, z, 0.3, PALETTE.ceramic, sc);
        }
      }
      const o = h.out;
      const e = eq(12, 7.4);
      if (o > 0.001 && e.show) {
        const px = lerp(lerp(7.9, 9.0, o), 11.2, e.m);
        const pz = lerp(2.1, -0.3, e.m);
        D.token(px, 0.4, pz, 0.8 * (0.2 + 0.8 * o), 'state', 'tail:pre');
      }
      if (e.ring) D.ring(11.2, 0.6, -0.3, 0.8, 0.8, e.lit);
    }
  }
  D.dim(0);
}

// Completed segments, the memory tree and its rulers (beats 3.14, 3.15, 3.18).
const SEGX = [7.0, 12.0];
const SEG_TILE = [SEGX[0] + (3 - 3.5) * 0.45, -2.3];
const ROOT_PRE_X = 7.0;
const sibX = TREE_X + 0.8;
const vsocket = (x, y, z, k, hex) => {
  D.push(x, y, z, Math.PI / 2, 0, 0, 1);
  D.outline(0, 0, 0, 0.6, 0.6, hex, 0.05);
  D.pop();
};
export function drawTree() {
  const bi = T.b;
  D.dim(1 - lit('backend'));
  // Segment trays at scale 0.5 with a break mark between them.
  for (let s = 0; s < 2; s++) {
    D.box('ti', SEGX[s], 0.2, -2.3, 3.7, 0.06, 0.55, PALETTE.titanium);
    for (let i = 0; i < 8; i++) {
      const sel = bi === 17 && s === 0 && i === 3;
      D.tile(SEGX[s] + (i - 3.5) * 0.45, 0.22, -2.3, 0.5, PALETTE.ceramic, true);
      if (sel) D.outline(SEGX[s] + (i - 3.5) * 0.45, 0.4, -2.3, 0.45, 0.45, PALETTE.heat, 0.06, ph(17, 0, 0.6));
    }
  }
  D.breakMark(9.5, 0.2, -2.3, 0.4);
  // Marker on the segment rail (beat 3.19).
  if (bi === 18) D.box('gold', SEG_TILE[0], 0.55, -2.8, 0.3, 0.3, 0.3, PALETTE.gold);
  D.dim(0);

  const tk = treeK();
  if (tk <= 0.001) return;
  D.dim(1 - lit('tree'));
  const col = [bi === 13 ? 1 : 0, bi === 17 ? 1 : 0];
  // Column, nodes, break mark, siblings, root.
  D.vbar(TREE_X, TREE_Z, 0.3, ROOT_Y * tk, PALETTE.gold, 0.06);
  D.box('matte', TREE_X, 3.1 * tk, TREE_Z, 0.6, 0.03, 0.06, PALETTE.onSurface, 0, 0, 0.35);
  D.box('matte', TREE_X, 3.25 * tk, TREE_Z, 0.6, 0.03, 0.06, PALETTE.onSurface, 0, 0, 0.35);
  // Cursor of the flush (beat 3.14): climbs twice. cursorY is the height of the cursor.
  const c1 = rp(13, 3.2, 4.2);
  const c2 = rp(13, 4.6, 5.6);
  const cur = bi === 13 ? (T.tb >= 4.6 ? c2 : c1) : 0;
  const cy = lerp(NODE_Y[0], ROOT_Y, cur);
  // Old read (beat 3.18): level j merges at 3.0 + 0.48 j.
  for (let j = 0; j < 5; j++) {
    const nlit = bi === 13 && cur > 0 && cur < 1 && Math.abs(cy - NODE_Y[j]) < 0.5 ? 0.8
      : bi === 17 && T.tb > 3.0 + 0.48 * j && T.tb < 3.0 + 0.48 * j + 0.4 ? 0.9 : 0;
    D.lbox('gold', TREE_X, NODE_Y[j] * tk, TREE_Z, 0.5, 0.3 * tk, 0.5, PALETTE.gold, PALETTE.goldBright, nlit);
    // Sibling token on the path.
    let slit = 0;
    let sx = sibX;
    if (bi === 13 && cur > 0 && cur < 1 && cy > NODE_Y[j] - 0.1 && cy < NODE_Y[j] + 0.55) slit = 0.8;
    if (bi === 17 && T.tb > 3.0) {
      const a = 3.0 + 0.48 * j;
      const q = ph(17, a, a + 0.4);
      sx = lerp(sibX, TREE_X, q);
      slit = q > 0 ? 0.6 : 0;
    }
    const gone = bi === 17 && T.tb > 3.0 + 0.48 * j + 0.4;
    if (!gone) D.token(sx, NODE_Y[j] * tk, TREE_Z, 0.5, 'state', 'sib:' + j, { lay: false, k: tk, lit: slit });
  }
  // Root token (pre then post) and the pre-state root token.
  const rc = bi >= 13 && (bi > 13 || T.tb > 5.4) ? 'root:post' : 'root:pre';
  D.token(TREE_X, ROOT_Y * tk, TREE_Z, 1.2, 'state', rc, { lay: false, k: tk });
  if (bi >= 17 && bi <= 18) D.token(ROOT_PRE_X, ROOT_Y, TREE_Z, 0.8, 'state', 'root:pre', { lay: false });
  // Depth ruler at x 11.2 (vertical), end ticks lit in beat 3.15.
  D.box('ti', 11.2, 3.2 * tk, TREE_Z, 0.1, 4.4 * tk, 0.2, PALETTE.titanium);
  const endLit = bi === 14 ? ph(14, 2.0, 2.4) : 0;
  for (let i = 0; i < 9; i++) {
    const end = i === 0 || i === 8;
    D.lbox('matte', 11.0, (1.0 + (i * 4.4) / 8) * tk, TREE_Z, 0.2, 0.04, 0.2, PALETTE.onSurface, PALETTE.goldBright, end ? endLit : 0);
  }
  D.breakMark(11.2, 3.2, TREE_Z + 0.12, 0.3);
  // Beat 3.14: the empty socket, the segment token and the cursor.
  if (bi === 13) {
    const enter = ph(13, 4.6, 5.6);
    const rise = ph(13, 2.0, 3.0);
    if (enter < 0.99) vsocket(SOCKET[0], SOCKET[1] + 0.3, SOCKET[2], 1, PALETTE.onSurface);
    const sx = lerp(lerp(9.5, 7.6, rise), SOCKET[0], enter);
    const sy = lerp(lerp(0.5, 1.0, rise), 1.0, enter);
    const sz = lerp(lerp(0.9, SOCKET[2], rise), SOCKET[2], enter);
    if (T.tb >= 2.0) D.token(sx, sy, sz, 0.6, 'state', 'seg:new', { lay: false });
    if (cur > 0 && cur < 1) D.box('glow', TREE_X - 0.3, cy, TREE_Z, 0.25, 0.25, 0.25, PALETTE.heat);
  } else if (bi > 13 && bi < 20 || (bi >= 20 && bi < 32)) {
    D.token(SOCKET[0], 1.0, SOCKET[2], 0.6, 'state', 'seg:new', { lay: false });
  }
  // Beat 3.18: the climbing token and the computed root.
  if (bi === 17) {
    const up = ph(17, 2.0, 3.0);
    const cl = rp(17, 3.0, 5.4);
    if (T.tb >= 2.0 && T.tb < 5.4) {
      const y = lerp(lerp(0.4, NODE_Y[0], up), ROOT_Y, cl);
      const x = lerp(lerp(SEG_TILE[0], TREE_X, up), TREE_X - 0.0, cl);
      const z = lerp(lerp(-2.3, TREE_Z, up), TREE_Z, cl);
      D.token(x, y, z, 0.6, 'state', 'seg:old', { lay: false });
    }
    const e = eq(17, 5.8);
    if (T.tb >= 5.4 && e.show) {
      D.token(lerp(8.1, ROOT_PRE_X, e.m), ROOT_Y, TREE_Z, 0.8 * (0.4 + 0.6 * ph(17, 5.4, 5.8)), 'state', 'root:pre', { lay: false });
    }
    if (e.ring) D.ring(ROOT_PRE_X, ROOT_Y, TREE_Z, 0.9, 0.9, e.lit);
  }
  if (bi >= 17 && bi <= 18) {
    const dk = bi === 17 ? ph(17, 7.4, 8.0) : 1;
    D.wire([[SEG_TILE[0], SEG_TILE[1]], [6.0, -2.65], [-3.9, -2.65], [-3.9, 0.8], [-4.9, 0.8]], dk, 0.45, PALETTE.onSurface);
  }
  D.dim(0);
}

// PAGE/ROM: two line trays at scale 0.4 (beats 3.19, 3.20).
export const pageX = (i) => 1.5 + (i - 3.5) * 0.36;
const PAGE_Z = [3.95, 4.55];
const PAGE_ROUTE = [[-4.7, 0.1, -0.6]];
export function drawPage() {
  const bi = T.b;
  D.dim(1 - lit('page'));
  D.housing(1.5, 4.2, 5, 1.6);
  for (let t = 0; t < 2; t++) {
    D.box('ti', 1.5, 0.33, PAGE_Z[t], 2.96, 0.06, 0.46, PALETTE.titanium);
  }
  const old = bi >= 18;
  const rom = bi >= 19 ? (bi === 19 ? ph(19, 0.4, 1.2) : 1) : 0;
  for (let i = 0; i < 8; i++) {
    if (old) D.tile(pageX(i), 0.34, PAGE_Z[0], 0.4, PALETTE.ceramic, true);
    else D.outline(pageX(i), 0.38, PAGE_Z[0], 0.32, 0.32, PALETTE.onSurface, 0.025);
    if (rom > 0.01) D.tile(pageX(i), 0.34, PAGE_Z[1], 0.4 * rom, PALETTE.gateViolet, true);
    else D.outline(pageX(i), 0.38, PAGE_Z[1], 0.32, 0.32, PALETTE.onSurface, 0.025);
  }
  if (bi === 18) {
    // Sweep across the eight tiles, then the connector climbs to the root token.
    D.sweepX(0.0, 3.0, 0.5, PAGE_Z[0], 0.4, rp(18, 2.6, 3.6));
    const ck = ph(18, 3.4, 4.2);
    D.wire([[3.1, 3.95], [4.6, 3.95], [4.6, -3.0], [9.0, -3.0]], ck, 0.45, PALETTE.onSurface);
    if (ck > 0.99) D.vbar(9.0, -3.0, 0.3, ROOT_Y * 0.98, PALETTE.gold, 0.05);
    // Branches to the bus lip, a 2-pip chip cell on the shared branch.
    const bk = ph(18, 4.0, 5.0);
    for (let i = 0; i < 8; i++) D.line(pageX(i), PAGE_Z[0], pageX(i), 4.2, bk, 0.45, PALETTE.onSurface, 0.04);
    D.line(-1.0, 4.2, 3.0, 4.2, bk, 0.45, PALETTE.onSurface, 0.05);
    if (bk > 0.01) {
      D.fcell(-0.4, 0.45, 4.2, 0.3, 'step', 0, PALETTE.ceramic, bk);
      D.box('matte', -0.45, 0.8 * bk, 4.2, 0.07, 0.02, 0.07, PALETTE.ink);
      D.box('matte', -0.35, 0.8 * bk, 4.2, 0.07, 0.02, 0.07, PALETTE.ink);
    }
    // Two requests, lane 0 then lane 1, land on different tiles.
    for (let r = 0; r < 2; r++) {
      const a = 6.0 + r * 1.2;
      const k = ph(18, a, a + 1.2);
      if (T.tb < a) continue;
      const lane = r === 0 ? -3.3 : -2.7;
      const tx = pageX(r === 0 ? 2 : 5);
      const route = [[-4.7, 0.1, -0.6], [lane, 0.1, -0.6], [lane, 0.1, 4.2], [tx, 0.4, 4.2], [tx, 0.5, PAGE_Z[0]]];
      tupleOn(route, k, 2, { s: 0.45 });
      if (k >= 1) D.ring(tx, 0.5, PAGE_Z[0], 0.4, 0.4, 0);
    }
  }
  if (bi === 19) {
    D.line(0.0, PAGE_Z[1], -1.5, 5.2, ph(19, 2.0, 3.2), 0.45, PALETTE.onSurface, 0.05);
    D.sweepX(0.0, 3.0, 0.5, PAGE_Z[1], 0.4, rp(19, 2.6, 3.6));
    const k = ph(19, 4.0, 5.4);
    if (T.tb >= 4.0) {
      const tx = pageX(3);
      const route = [[-4.7, 0.1, -0.6], [-3.3, 0.1, -0.6], [-3.3, 0.1, 4.2], [tx, 0.4, 4.2], [tx, 0.5, PAGE_Z[1]]];
      tupleOn(route, k, 4, { s: 0.45 });
      if (k >= 1) D.ring(tx, 0.5, PAGE_Z[1], 0.4, 0.4, k >= 1 ? 0.3 * (1 - rp(19, 5.4, 5.8)) : 0);
    }
  }
  if (bi >= 19) D.token(-1.5, 0.3, 5.2, 0.8, 'program', 'prog:main');
  D.dim(0);
}

// Row budget: titanium frame from x -8 to 8 at z 5.75 with two gold fills.
export function drawBudget() {
  D.dim(0);
  D.box('ti', 0, 0.06, 5.75, 16, 0.12, 0.3, PALETTE.titanium);
  const flush = bi_ge(13) ? (T.b === 13 ? ph(13, 5.6, 6.2) : 1) : 0;
  const old = bi_ge(17) ? (T.b === 17 ? ph(17, 7.4, 8.0) : 1) : 0;
  if (flush > 0.001) D.box('gold', -8 + 1.075 * flush, 0.14, 5.75, 2.15 * flush, 0.1, 0.22, PALETTE.gold);
  if (old > 0.001) D.box('gold', -8 + 2.15 + 0.58 * old, 0.14, 5.75, 1.16 * old, 0.1, 0.22, PALETTE.gold);
}
const bi_ge = (n) => T.b >= n;
