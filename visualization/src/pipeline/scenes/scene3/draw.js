// Immediate-mode drawing for scene 3. Six instanced pools hold every box, hex disc and arc of the
// scene. `begin()` empties them, the scene redraws everything from time alone, `end()` uploads.
import * as THREE from 'three/webgpu';
import { PALETTE, FAMILY, materials, digestPattern, lerp } from '../../theme.js';

const CAP = { matte: 3400, ti: 700, gold: 900, glow: 900, hex: 700, arc: 160 };
const pools = {};
let dimK = 0;
let geos = [];
let mats = [];

const M = new THREE.Matrix4();
const L = new THREE.Matrix4();
const Q = new THREE.Quaternion();
const E = new THREE.Euler();
const V = new THREE.Vector3();
const S = new THREE.Vector3();
const C = new THREE.Color();
const WARM = new THREE.Color();
const tmpC = new THREE.Color();
const FS = Array.from({ length: 8 }, () => new THREE.Matrix4());
let depth = 0;

export function build(group) {
  WARM.setHex(PALETTE.paperWarm);
  const unit = new THREE.BoxGeometry(1, 1, 1);
  const hexg = new THREE.CylinderGeometry(0.5, 0.5, 1, 6);
  const arcg = new THREE.TorusGeometry(1, 0.1, 6, 10, (66 * Math.PI) / 180);
  arcg.rotateX(-Math.PI / 2);
  geos = [unit, hexg, arcg];
  const white = (m) => { m.color.setRGB(1, 1, 1); return m; };
  const glowMat = new THREE.MeshBasicMaterial();
  glowMat.color.setRGB(1, 1, 1);
  mats = [white(materials.ceramic()), white(materials.titanium()), white(materials.gold()), glowMat, white(materials.ceramic()), white(materials.gold())];
  const spec = { matte: [unit, mats[0]], ti: [unit, mats[1]], gold: [unit, mats[2]], glow: [unit, mats[3]], hex: [hexg, mats[4]], arc: [arcg, mats[5]] };
  for (const k of Object.keys(spec)) {
    const mesh = new THREE.InstancedMesh(spec[k][0], spec[k][1], CAP[k]);
    mesh.frustumCulled = false;
    mesh.count = 0;
    mesh.instanceMatrix.setUsage(THREE.DynamicDrawUsage);
    mesh.setColorAt(0, C.setRGB(1, 1, 1));
    group.add(mesh);
    pools[k] = { mesh, n: 0, cap: CAP[k] };
  }
}

export function dispose() {
  for (const k of Object.keys(pools)) {
    pools[k].mesh.removeFromParent();
    pools[k].mesh.dispose();
    delete pools[k];
  }
  geos.forEach((g) => g.dispose());
  mats.forEach((m) => m.dispose());
  geos = []; mats = [];
}

export function stats() {
  let n = 0;
  for (const k of Object.keys(pools)) n += pools[k].n;
  return n;
}

export function begin() {
  for (const k of Object.keys(pools)) pools[k].n = 0;
  depth = 0;
  dimK = 0;
  FS[0].identity();
}

export function end() {
  for (const k of Object.keys(pools)) {
    const p = pools[k];
    p.mesh.count = p.n;
    p.mesh.visible = p.n > 0;
    p.mesh.instanceMatrix.needsUpdate = true;
    if (p.mesh.instanceColor) p.mesh.instanceColor.needsUpdate = true;
  }
}

// Region dimming: 0 lit, 1 fully dimmed (colour lerp 0.6 toward paperWarm, no opacity).
export function dim(k) { dimK = k; }
export const getDim = () => dimK;

export function push(x = 0, y = 0, z = 0, rx = 0, ry = 0, rz = 0, s = 1) {
  E.set(rx, ry, rz);
  Q.setFromEuler(E);
  L.compose(V.set(x, y, z), Q, S.set(s, s, s));
  FS[depth + 1].multiplyMatrices(FS[depth], L);
  depth++;
}
export function pop() { depth--; }

function put(pl, x, y, z, w, h, d, hex, rx, ry, rz, hex2, mix) {
  if (pl.n >= pl.cap || w <= 0 || h <= 0 || d <= 0) return;
  E.set(rx, ry, rz);
  Q.setFromEuler(E);
  L.compose(V.set(x, y, z), Q, S.set(w, h, d));
  M.multiplyMatrices(FS[depth], L);
  pl.mesh.setMatrixAt(pl.n, M);
  C.setHex(hex);
  if (mix > 0) C.lerp(tmpC.setHex(hex2), mix);
  if (dimK > 0) C.lerp(WARM, 0.6 * dimK);
  pl.mesh.setColorAt(pl.n, C);
  pl.n++;
}

// ---- raw primitives -------------------------------------------------------------------------
export const box = (pool, x, y, z, w, h, d, hex, ry = 0, rx = 0, rz = 0) => put(pools[pool], x, y, z, w, h, d, hex, rx, ry, rz, 0, 0);
// A box that glows toward `bright` by k (state only): drawn unlit once k is above 0.
export function lbox(pool, x, y, z, w, h, d, hex, bright, k, ry = 0) {
  if (k > 0.02) put(pools.glow, x, y, z, w, h, d, hex, 0, ry, 0, bright, Math.min(1, k));
  else put(pools[pool], x, y, z, w, h, d, hex, 0, ry, 0, 0, 0);
}

export const cell = (x, y, z, s, hex = PALETTE.ceramic, k = 1) => {
  if (k <= 0.001) return;
  put(pools.matte, x, y + (s * k) / 2, z, s * k, s * k, s * k, hex, 0, 0, 0, 0, 0);
};

// A cell that lights toward a family (the step in progress).
export const fcell = (x, y, z, s, fam, lit, base = PALETTE.ceramic, k = 1) => {
  if (k <= 0.001) return;
  const f = FAMILY[fam];
  if (lit > 0.02) put(pools.glow, x, y + (s * k) / 2, z, s * k, s * k, s * k, f.hex, 0, 0, 0, f.bright, lit);
  else put(pools.matte, x, y + (s * k) / 2, z, s * k, s * k, s * k, base, 0, 0, 0, 0, 0);
};

// Thin line on the bench from a to b (horizontal), grown to fraction k from a.
export function line(x0, z0, x1, z1, k = 1, y = 0.05, hex = PALETTE.onSurface, w = 0.06) {
  if (k <= 0.001) return;
  const dx = (x1 - x0) * k;
  const dz = (z1 - z0) * k;
  const len = Math.hypot(dx, dz);
  if (len < 1e-4) return;
  put(pools.matte, x0 + dx / 2, y, z0 + dz / 2, len, 0.04, w, hex, 0, Math.atan2(-dz, dx), 0, 0, 0);
}
// Polyline connector with fraction k of its total length drawn.
export function wire(pts, k = 1, y = 0.05, hex = PALETTE.onSurface) {
  let tot = 0;
  for (let i = 1; i < pts.length; i++) tot += Math.hypot(pts[i][0] - pts[i - 1][0], pts[i][1] - pts[i - 1][1]);
  let left = tot * k;
  for (let i = 1; i < pts.length && left > 0; i++) {
    const a = pts[i - 1];
    const b = pts[i];
    const sg = Math.hypot(b[0] - a[0], b[1] - a[1]);
    const f = sg ? Math.min(1, left / sg) : 0;
    line(a[0], a[1], b[0], b[1], f, y, hex);
    left -= sg;
  }
}

// Rectangle outline (four thin boxes) around a w by d footprint centred at x, z.
export function outline(x, y, z, w, d, hex = PALETTE.onSurface, t = 0.05, k = 0, bright = PALETTE.goldBright) {
  const pool = k > 0.02 ? 'glow' : 'matte';
  const h = 0.04;
  const m = k > 0.02 ? k : 0;
  put(pools[pool], x, y, z - d / 2, w + t, h, t, hex, 0, 0, 0, bright, m);
  put(pools[pool], x, y, z + d / 2, w + t, h, t, hex, 0, 0, 0, bright, m);
  put(pools[pool], x - w / 2, y, z, t, h, d - t, hex, 0, 0, 0, bright, m);
  put(pools[pool], x + w / 2, y, z, t, h, d - t, hex, 0, 0, 0, bright, m);
}
// Gold equality outline: 10 percent larger than the object, lit k.
export function ring(x, y, z, w, d, k) {
  outline(x, y, z, w * 1.1, d * 1.1, PALETTE.gold, 0.05, k);
}

// Housing: ceramic box 0.3 high with a titanium rim line.
export function housing(x, z, w, d, h = 0.3, hex = PALETTE.ceramicEdge, k = 1) {
  if (k <= 0.001) return;
  put(pools.matte, x, (h * k) / 2, z, w, h * k, d, hex, 0, 0, 0, 0, 0);
  const t = 0.05;
  const y = h * k + 0.01;
  put(pools.ti, x, y, z - d / 2 + t / 2, w, 0.03, t, PALETTE.titanium, 0, 0, 0, 0, 0);
  put(pools.ti, x, y, z + d / 2 - t / 2, w, 0.03, t, PALETTE.titanium, 0, 0, 0, 0, 0);
  put(pools.ti, x - w / 2 + t / 2, y, z, t, 0.03, d - 2 * t, PALETTE.titanium, 0, 0, 0, 0, 0);
  put(pools.ti, x + w / 2 - t / 2, y, z, t, 0.03, d - 2 * t, PALETTE.titanium, 0, 0, 0, 0, 0);
}

// Memory record: tile 0.8 by 0.12 by 0.8 with one ink bar. s scales it; hex overrides the colour.
export function tile(x, y, z, s = 1, hex = PALETTE.ceramic, bar = true, lit = 0, bright = PALETTE.heatBright) {
  if (s <= 0.001) return;
  lbox('matte', x, y + 0.06 * s, z, 0.8 * s, 0.12 * s, 0.8 * s, hex, bright, lit);
  if (bar) put(pools.matte, x, y + 0.13 * s, z, 0.5 * s, 0.02 * s, 0.1 * s, PALETTE.ink, 0, 0, 0, 0, 0);
}

// Digest token: hex disc, family rim, ceramic cap, 5 by 5 ink dots. Flat on the bench or upright.
// opts: lay (default true), ring 0..1 (five gold arc segments), lit 0..1.
export function token(x, y, z, size, family, content, o = {}) {
  const k = o.k ?? 1;
  if (k <= 0.001) return;
  const sz = size * k;
  const lay = o.lay !== false;
  const f = FAMILY[family] || FAMILY.state;
  push(x, y + (lay ? 0.09 * sz : 0), z, lay ? 0 : Math.PI / 2, 0, 0, 1);
  const r = 0.5 * sz;
  const h = 0.18 * sz;
  const lit = o.lit || 0;
  if (lit > 0.02) put(pools.glow, 0, 0, 0, 2 * r, h * 0.6, 2 * r, f.hex, 0, Math.PI / 6, 0, f.bright, lit);
  else put(pools.hex, 0, 0, 0, 2 * r, h * 0.6, 2 * r, f.hex, 0, Math.PI / 6, 0, 0, 0);
  put(pools.hex, 0, h * 0.5, 0, 1.78 * r, h * 0.45, 1.78 * r, PALETTE.ceramic, 0, Math.PI / 6, 0, 0, 0);
  const bits = digestPattern(content);
  const sp = (r * 1.45) / 5;
  const dd = sp * 0.62;
  for (let i = 0; i < 25; i++) {
    if (!bits[i]) continue;
    put(pools.matte, ((i % 5) - 2) * sp, h * 0.76, (Math.floor(i / 5) - 2) * sp, dd, h * 0.1, dd, PALETTE.ink, 0, 0, 0, 0, 0);
  }
  if (o.ring > 0.01) {
    const R = r * 1.12;
    for (let i = 0; i < 5; i++) put(pools.arc, 0, 0, 0, R * o.ring, R * o.ring, R * o.ring, PALETTE.gold, 0, (i * 72 * Math.PI) / 180 + 0.1, 0, 0, 0);
  }
  pop();
}

// Hash block (kit): gold box 2.4 by 1.0 by 1.2 with slots, scaled s, centred at x, z, base y.
export function hashBlock(x, y, z, s, lit = 0) {
  if (s <= 0.001) return;
  lbox('gold', x, y + 0.5 * s, z, 2.4 * s, 1.0 * s, 1.2 * s, PALETTE.gold, PALETTE.goldBright, lit * 0.8);
  put(pools.matte, x - 1.2 * s, y + 0.5 * s, z, 0.1 * s, 0.5 * s, 0.5 * s, PALETTE.ink, 0, 0, 0, 0, 0);
  put(pools.matte, x + 1.2 * s, y + 0.5 * s, z, 0.1 * s, 0.5 * s, 0.5 * s, PALETTE.ink, 0, 0, 0, 0, 0);
}

// Verification bar crossing a footprint: from -x to +x over fraction f (0 to 1). Only drawn inside (0, 1).
export function sweepX(x0, x1, y, z, depthZ, f, hold = false) {
  if (f <= 0 || (f >= 1 && !hold)) return;
  put(pools.glow, lerp(x0, x1, f), y, z, 0.08, 0.04, depthZ, PALETTE.goldBright, 0, 0, 0, 0, 0);
}
// A bar crossing along z.
export function sweepZ(z0, z1, y, x, width, f, hold = false) {
  if (f <= 0 || (f >= 1 && !hold)) return;
  put(pools.glow, x, y, lerp(z0, z1, f), width, 0.04, 0.08, PALETTE.goldBright, 0, 0, 0, 0, 0);
}

// Refusal hatch: three bars at 45 degrees across a footprint.
export function hatch(x, y, z, k = 1) {
  if (k <= 0.001) return;
  for (let i = -1; i <= 1; i++) put(pools.glow, x + i * 0.17, y, z + i * 0.17, 0.5 * k, 0.03, 0.08, PALETTE.errorRed, 0, Math.PI / 4, 0, 0, 0);
}

// Bus tuple: five cells in a row (pitch 0.45 times s), chip cell first with `chip` ink pips.
// yaw 0 runs along +x, yaw -PI/2 along +z.
export function tuple(x, y, z, chip, o = {}) {
  const s = o.s ?? 0.7;
  const k = o.k ?? 1;
  if (k <= 0.001) return;
  const yaw = o.yaw ?? 0;
  const flat = o.flat ? 0.2 : 1;
  push(x, y, z, 0, yaw, 0, 1);
  const cs = 0.4 * s * k;
  const lit = o.lit ?? 1;
  for (let i = 0; i < 5; i++) {
    const cx = (i - 2) * 0.45 * s;
    if (i === 0 && o.chipFam !== null) {
      const fam = o.chipFam || 'step';
      const f = FAMILY[fam];
      if (lit > 0.02) put(pools.glow, cx, cs * flat / 2, 0, cs, cs * flat, cs, f.hex, 0, 0, 0, f.bright, lit);
      else put(pools.matte, cx, cs * flat / 2, 0, cs, cs * flat, cs, f.hex, 0, 0, 0, 0, 0);
      for (let p = 0; p < chip; p++) put(pools.matte, cx + ((p % 2) - 0.5) * 0.14 * s * (chip > 1 ? 1 : 0), cs * flat + 0.01, (Math.floor(p / 2) - (chip > 2 ? 0.5 : 0)) * 0.14 * s * (chip > 2 ? 1 : 0), 0.08 * s, 0.02, 0.08 * s, PALETTE.ink, 0, 0, 0, 0, 0);
    } else {
      put(pools.matte, cx, cs * flat / 2, 0, cs, cs * flat, cs, o.hex ?? PALETTE.ceramic, 0, 0, 0, 0, 0);
    }
  }
  pop();
}

// Ruler: titanium strip with ticks. n ticks evenly spaced from x0 to x1 at z.
export function ruler(x0, x1, y, z, n, o = {}) {
  const w = x1 - x0;
  put(pools.ti, (x0 + x1) / 2, y, z, w, 0.1, 0.3, PALETTE.titanium, 0, 0, 0, 0, 0);
  for (let i = 0; i < n; i++) {
    const lit = o.litTicks;
    put(pools.matte, x0 + (n > 1 ? (i / (n - 1)) * w : 0), y + 0.06, z, 0.04, 0.03, 0.2, lit ? PALETTE.goldBright : PALETTE.onSurface, 0, 0, 0, 0, 0);
  }
}

// Break mark: two parallel strokes at 20 degrees.
export function breakMark(x, y, z, w = 0.6) {
  box('matte', x - 0.08, y, z, w, 0.03, 0.05, PALETTE.onSurface, 0.35);
  box('matte', x + 0.08, y, z, w, 0.03, 0.05, PALETTE.onSurface, 0.35);
}

// Vertical bar from y0 to y1 at x, z.
export function vbar(x, z, y0, y1, hex = PALETTE.gold, w = 0.08) {
  if (y1 <= y0) return;
  put(pools.gold, x, (y0 + y1) / 2, z, w, y1 - y0, w, hex, 0, 0, 0, 0, 0);
}
