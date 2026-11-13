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
  zoneOff: M.zoneOff, zoneOn: M.zoneOn,
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

// ---- face atlas ------------------------------------------------------------------------------------
// Every shared token face is a cell of one atlas texture (12 by 12 cells of 160 px, a second page when full),
// so all shared tokens use one face material per page and one unit face geometry per cell. A cell is a pure
// function of its content string; the cell index only depends on the order of first use.
const ATLAS_PX = 2048, CELL_PX = 160, GRID = ATLAS_PX / CELL_PX | 0;
const atlasPages = [];
const atlasCells = new Map();
function atlasCell(content) {
  const key = String(content);
  let c = atlasCells.get(key);
  if (c) return c;
  let page = atlasPages[atlasPages.length - 1];
  if (!page || page.n >= GRID * GRID) {
    const cv = document.createElement('canvas');
    cv.width = cv.height = ATLAS_PX;
    const g = cv.getContext('2d');
    g.fillStyle = hexCss(PALETTE.ceramic);
    g.fillRect(0, 0, ATLAS_PX, ATLAS_PX);
    const tex = new THREE.CanvasTexture(cv);
    tex.colorSpace = THREE.SRGBColorSpace;
    tex.anisotropy = 4;
    const mat = new THREE.MeshBasicMaterial({ map: tex, toneMapped: false });
    mat.userData.shared = true;
    page = { cv, g, tex, mat, n: 0 };
    atlasPages.push(page);
  }
  const col = page.n % GRID, row = (page.n / GRID) | 0;
  page.n++;
  const g = page.g, x0 = col * CELL_PX, y0 = row * CELL_PX;
  g.fillStyle = hexCss(PALETTE.ceramic);
  g.fillRect(x0, y0, CELL_PX, CELL_PX);
  g.fillStyle = hexCss(PALETTE.ink);
  const bits = digestPattern(key);
  const cell = CELL_PX / 6;
  for (let i = 0; i < 25; i++) {
    if (!bits[i]) continue;
    g.beginPath();
    g.arc(x0 + cell * (1 + (i % 5)), y0 + cell * (1 + Math.floor(i / 5)), cell * 0.3, 0, Math.PI * 2);
    g.fill();
  }
  page.tex.needsUpdate = true;
  // Unit plane whose uv covers this cell (canvas row 0 is the top of the texture).
  const geo = new THREE.PlaneGeometry(1, 1);
  const u0 = x0 / ATLAS_PX, u1 = (x0 + CELL_PX) / ATLAS_PX, v1 = 1 - y0 / ATLAS_PX, v0 = 1 - (y0 + CELL_PX) / ATLAS_PX;
  geo.setAttribute('uv', new THREE.Float32BufferAttribute([u0, v1, u1, v1, u0, v0, u1, v0], 2));
  c = { geo, mat: page.mat };
  atlasCells.set(key, c);
  return c;
}

// Shared token resources. Geometry and materials are cached by shape, family and content, so thirty
// tokens build one body geometry, one face geometry, one rim material per family, one cap material and
// one face material per content string. A token that a scene tints on its own passes `own: true` and
// gets private copies of the three material kinds (the cached geometry stays shared).
const tokenCache = { geo: new Map(), rim: new Map(), cap: new Map(), face: new Map() };
const memo = (map, key, make) => { let v = map.get(key); if (!v) { v = make(); map.set(key, v); } return v; };
const tokenMaterials = (family, content, overlay, own) => {
  const mk = () => ({
    rim: materials.family(family, { roughness: 0.45, metalness: 0.2 }),
    cap: materials.ceramic({ roughness: 0.7 }),
    face: new THREE.MeshBasicMaterial({ map: faceTexture(content), toneMapped: false }),
  });
  let out;
  if (own) out = mk();
  else out = {
    rim: memo(tokenCache.rim, `${family}|${overlay}`, () => materials.family(family, { roughness: 0.45, metalness: 0.2 })),
    cap: memo(tokenCache.cap, `${overlay}`, () => materials.ceramic({ roughness: 0.7 })),
    face: atlasCell(content).mat,
  };
  if (overlay) for (const m of Object.values(out)) m.depthTest = false;
  if (own) for (const m of Object.values(out)) m.userData.shared = false;
  else { out.rim.userData.shared = true; out.cap.userData.shared = true; }
  return out;
};

// Returns a Group (radius 0.5 * size, thickness 0.18 * size, face toward +z).
// group.userData.setContent(str) changes the face. group.userData.anchor is an Object3D above the token.
export function makeDigestToken({ family = 'state', content = '', size = 1, overlay = false, own = false } = {}) {
  const grp = new THREE.Group();
  const r = 0.5 * size;
  const h = 0.18 * size;
  if (overlay) own = true;   // the carrier changes its faces in place
  const mt = tokenMaterials(family, content, overlay, own);
  const bodyGeo = memo(tokenCache.geo, `b${size}`, () => new THREE.CylinderGeometry(r, r, h, 6));
  const body = new THREE.Mesh(bodyGeo, [mt.rim, mt.cap, mt.cap]);
  body.rotation.x = Math.PI / 2;
  body.rotation.y = Math.PI / 6;
  grp.add(body);
  const faceGeo = own ? memo(tokenCache.geo, `f${size}`, () => new THREE.PlaneGeometry(r * 1.45, r * 1.45)) : atlasCell(content).geo;
  const face = new THREE.Mesh(faceGeo, mt.face);
  if (!own) face.scale.setScalar(r * 1.45);
  face.position.z = h / 2 + 0.004 * size;
  grp.add(face);
  if (overlay) {
    // Drawn over everything (token carrier).
    body.renderOrder = 998;
    face.renderOrder = 999;
  }
  const anchor = new THREE.Object3D();
  anchor.position.y = r * 1.05;
  grp.add(anchor);
  grp.userData.anchor = anchor;
  grp.userData.radius = r;
  grp.userData.setContent = (c) => {
    if (own) {
      const tex = faceTexture(c);
      if (face.material.map !== tex) { face.material.map = tex; face.material.needsUpdate = true; }
      return;
    }
    const cellOf = atlasCell(c);
    face.geometry = cellOf.geo;
    face.material = cellOf.mat;
  };
  return grp;
}
