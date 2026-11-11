// Shared palette, easing helpers, materials and the digest-token object for the pipeline page.
// Canvas colours come from PALETTE (designers/theme.js). DOM colours come from styles.css variables.
import * as THREE from 'three/webgpu';
import { PALETTE as BASE_PALETTE } from '../designers/theme.js';
import { M3_CANVAS as M } from './m3palette.js';

// Canvas colours are the M3 dark tonal values (m3.css, seed #d4af37), mapped onto the names
// the earlier palette used. Prover side = lighter dark surface, chain side = deepest surface.
export const PALETTE = {
  ...BASE_PALETTE,
  paper: M.paper,                 // prover ground: surface-container-high
  paperWarm: M.paperWarm,         // prover bench slabs: surface-container-highest
  wellSlate: M.wellSlate,         // chain ground: surface-container-lowest
  graphite: M.graphite,           // chain bench slabs: surface-container-low
  ceramic: M.ceramic, ceramicAlabaster: M.ceramic, ceramicEdge: M.ceramicEdge,
  titanium: M.titanium, titaniumSilver: M.titaniumSilver,
  ink: M.ink, inkSoft: M.inkSoft, onSurface: M.onSurface,
  gold: M.gold, goldBright: M.goldBright,
  gateViolet: M.program, violet: M.program,
  steel: M.input, steelBright: M.inputBright, cyan: M.inputBright,
  resolve: M.outcome, resolveBright: M.outcomeBright, envCyan: M.outcomeBright,
  heat: M.step, heatBright: M.stepBright,
  errorRed: M.errorRed,
};
const hexCss = (n) => '#' + n.toString(16).padStart(6, '0');

// One hue, one meaning. `hex` is the canvas colour, `bright` the lit variant.
export const FAMILY = {
  program: { hex: PALETTE.gateViolet, bright: M.programBright, css: 'var(--md-custom-program)' },
  input: { hex: PALETTE.steel, bright: PALETTE.steelBright, css: 'var(--md-custom-input)' },
  outcome: { hex: PALETTE.resolve, bright: PALETTE.resolveBright, css: 'var(--md-custom-outcome)' },
  step: { hex: PALETTE.heat, bright: PALETTE.heatBright, css: 'var(--md-custom-step)' },
  state: { hex: PALETTE.gold, bright: PALETTE.goldBright, css: 'var(--md-sys-color-primary)' },
  error: { hex: PALETTE.errorRed, bright: PALETTE.errorRed, css: 'var(--md-sys-color-error)' },
  neutral: { hex: PALETTE.ceramicEdge, bright: PALETTE.ceramicEdge, css: 'var(--md-sys-color-outline)' },
};

// ---- pure-function animation helpers -------------------------------------------------
export const clamp01 = (x) => (x < 0 ? 0 : x > 1 ? 1 : x);
export const lerp = (a, b, k) => a + (b - a) * k;
export const easeInOut = (k) => {
  k = clamp01(k);
  return k < 0.5 ? 4 * k * k * k : 1 - Math.pow(-2 * k + 2, 3) / 2;
};
// Eased progress of t between seconds a and b (0 before a, 1 after b).
export const phase = (t, a, b) => easeInOut((t - a) / (b - a));
// Linear progress, for counters and sweeps.
export const ramp = (t, a, b) => clamp01((t - a) / (b - a));

// ---- materials: each call returns a fresh instance, so a scene can tint its own copy ----
export const materials = {
  ceramic: (o = {}) => new THREE.MeshStandardMaterial({ color: PALETTE.ceramic, roughness: 0.85, metalness: 0, ...o }),
  paper: (o = {}) => new THREE.MeshStandardMaterial({ color: PALETTE.paperWarm, roughness: 0.95, metalness: 0, ...o }), // prover bench
  slate: (o = {}) => new THREE.MeshStandardMaterial({ color: PALETTE.graphite, roughness: 0.9, metalness: 0.05, ...o }),
  titanium: (o = {}) => new THREE.MeshStandardMaterial({ color: PALETTE.titanium, roughness: 0.42, metalness: 0.55, ...o }),
  gold: (o = {}) => new THREE.MeshStandardMaterial({ color: PALETTE.gold, roughness: 0.35, metalness: 0.7, ...o }),
  ink: (o = {}) => new THREE.MeshStandardMaterial({ color: PALETTE.ink, roughness: 0.8, metalness: 0, ...o }),
  // Family colour. `lit` in 0..1 adds emissive: state only (active rule, passing check).
  family: (name, { lit = 0, ...o } = {}) => {
    const f = FAMILY[name] || FAMILY.neutral;
    const m = new THREE.MeshStandardMaterial({ color: f.hex, roughness: 0.6, metalness: 0.05, ...o });
    if (lit > 0) { m.emissive = new THREE.Color(f.bright); m.emissiveIntensity = lit; }
    return m;
  },
};

export function makeCube(size = 1, material = materials.ceramic()) {
  const m = new THREE.Mesh(new THREE.BoxGeometry(size, size, size), material);
  return m;
}

// ---- digest tokens ---------------------------------------------------------------------
// A digest is shown as a hexagonal token with a 5 by 5 dot pattern computed from the content
// string it stands for. Equal strings give equal faces. No digest value is ever invented.
function hash32(str) {
  let h = 2166136261 >>> 0;
  for (let i = 0; i < str.length; i++) {
    h ^= str.charCodeAt(i);
    h = Math.imul(h, 16777619) >>> 0;
  }
  return h >>> 0;
}

export function digestPattern(content) {
  let s = (hash32(String(content)) ^ 0x9e3779b9) >>> 0 || 1;
  const bits = [];
  for (let i = 0; i < 25; i++) {
    s = (s ^ (s << 13)) >>> 0;
    s = (s ^ (s >>> 17)) >>> 0;
    s = (s ^ (s << 5)) >>> 0;
    bits.push((s >>> 9) & 1);
  }
  if (!bits.some(Boolean)) bits[12] = 1;
  return bits;
}

const faceCache = new Map();
function faceTexture(content) {
  const key = String(content);
  let tex = faceCache.get(key);
  if (tex) return tex;
  const px = 160;
  const cv = document.createElement('canvas');
  cv.width = cv.height = px;
  const g = cv.getContext('2d');
  g.fillStyle = hexCss(PALETTE.ceramic);
  g.fillRect(0, 0, px, px);
  g.fillStyle = hexCss(PALETTE.ink);
  const bits = digestPattern(key);
  const cell = px / 6;
  for (let i = 0; i < 25; i++) {
    if (!bits[i]) continue;
    const cx = cell * (0.5 + (i % 5) + 0.5);
    const cy = cell * (0.5 + Math.floor(i / 5) + 0.5);
    g.beginPath();
    g.arc(cx, cy, cell * 0.3, 0, Math.PI * 2);
    g.fill();
  }
  tex = new THREE.CanvasTexture(cv);
  tex.colorSpace = THREE.SRGBColorSpace;
  tex.anisotropy = 4;
  faceCache.set(key, tex);
  return tex;
}

// Returns a Group (radius 0.5 * size, thickness 0.18 * size, face toward +z).
// group.userData.setContent(str) changes the face. group.userData.anchor is an Object3D above the token.
export function makeDigestToken({ family = 'state', content = '', size = 1, overlay = false } = {}) {
  const grp = new THREE.Group();
  const r = 0.5 * size;
  const h = 0.18 * size;
  const rimMat = materials.family(family, { roughness: 0.45, metalness: 0.2 });
  const capMat = materials.ceramic({ roughness: 0.7 });
  const body = new THREE.Mesh(new THREE.CylinderGeometry(r, r, h, 6), [rimMat, capMat, capMat]);
  body.rotation.x = Math.PI / 2;
  body.rotation.y = Math.PI / 6;
  grp.add(body);
  const faceMat = new THREE.MeshBasicMaterial({ map: faceTexture(content), toneMapped: false });
  const face = new THREE.Mesh(new THREE.PlaneGeometry(r * 1.45, r * 1.45), faceMat);
  face.position.z = h / 2 + 0.004 * size;
  grp.add(face);
  if (overlay) {
    // Drawn over everything (token carrier).
    for (const m of [rimMat, capMat, faceMat]) m.depthTest = false;
    body.renderOrder = 998;
    face.renderOrder = 999;
  }
  const anchor = new THREE.Object3D();
  anchor.position.y = r * 1.05;
  grp.add(anchor);
  grp.userData.anchor = anchor;
  grp.userData.radius = r;
  grp.userData.setContent = (c) => {
    const tex = faceTexture(c);
    if (faceMat.map !== tex) { faceMat.map = tex; faceMat.needsUpdate = true; }
  };
  return grp;
}
