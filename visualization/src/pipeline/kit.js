// Shared kit: every object that more than one scene draws. Each builder is a pure function of its
// parameters and returns a Group with userData.setState(...). setState sets the whole look of the
// object from its arguments alone, so a scene's seek stays deterministic: call it every frame.
// Sizes are scene units (STORYBOARD section 0.4). Colours come from theme.js only.
import * as THREE from 'three/webgpu';
import { PALETTE, FAMILY, materials, makeDigestToken, clamp01, lerp } from './theme.js';

export { makeDigestToken };

const rad = (d) => (d * Math.PI) / 180;
// Geometry and static materials are cached across every kit object. A material that setState rewrites
// per frame (pins, edge plate, closing arc) is not shared; a material marked userData.shared is, and a
// scene that tints a group on its own clones it first (scene 4 makeDimmer does).
const geoCache = new Map();
const sharedGeo = (key, make) => { let g = geoCache.get(key); if (!g) { g = make(); geoCache.set(key, g); } return g; };
const matCache = new Map();
const sharedMat = (key, make) => { let m = matCache.get(key); if (!m) { m = make(); m.userData.shared = true; matCache.set(key, m); } return m; };
const box = (w, h, d, mat) => new THREE.Mesh(sharedGeo(`box${w}|${h}|${d}`, () => new THREE.BoxGeometry(w, h, d)), mat);
const lit = (mat, family, amount) => {
  const f = FAMILY[family] || FAMILY.state;
  mat.emissive.setHex(f.bright);
  mat.emissiveIntensity = amount;
};

// Disposes the geometries and materials created by a kit builder (digest face textures are shared and stay).
function attachDispose(grp) {
  grp.userData.dispose = () => {
    grp.traverse((o) => {
      if (o.geometry) o.geometry.dispose();
      if (o.material) (Array.isArray(o.material) ? o.material : [o.material]).forEach((m) => m.dispose());
    });
  };
}

// ---- state token -------------------------------------------------------------------------------
// A boundary digest (pre_d, post_d): a gold digest token whose rim carries a cap ring of five arc
// segments, one per header field. The ring opens into a row of the five fields.
export const HEADER_FIELDS = ['outcome_tag', 'outcome_payload', 'input_d', 'deps_d', 'body_d'];

export function makeStateToken({ content = 'state:main', size = 1, family = 'state' } = {}) {
  const grp = makeDigestToken({ family, content, size });
  const r = 0.5 * size;
  const R = r * 1.12;
  const tube = 0.06 * size;
  const pitch = 1.25 * size;
  const rowY = r * 2.3;
  const segs = [];
  const anchors = [];
  const mats = [];
  for (let i = 0; i < 5; i++) {
    const geo = sharedGeo(`seg${R}|${tube}`, () => {
      const g2 = new THREE.TorusGeometry(R, tube, 8, 14, rad(66));
      g2.rotateZ(-rad(33));
      g2.translate(-R, 0, 0);
      return g2;
    });
    const mat = materials.gold();
    mats.push(mat);
    const g = new THREE.Group();
    g.add(new THREE.Mesh(geo, mat));
    g.position.z = 0.1 * size;
    grp.add(g);
    segs.push(g);
    const a = new THREE.Object3D();
    grp.add(a);
    anchors.push(a);
  }
  // open: 0 closed ring, 1 row of five fields. highlight: index of one segment, -2 for all, -1 none.
  // lit: 0..1 emissive on the highlighted segments (a passing check).
  grp.userData.setState = ({ open = 0, lit: l = 0, highlight = -1 } = {}) => {
    const o = clamp01(open);
    for (let i = 0; i < 5; i++) {
      const th = rad(90 - i * 72);
      const cx = R * Math.cos(th), cy = R * Math.sin(th);
      const ox = (i - 2) * pitch, oy = rowY;
      const g = segs[i];
      g.position.set(lerp(cx, ox, o), lerp(cy, oy, o), 0.1 * size);
      g.rotation.z = lerp(th, Math.PI / 2, o);
      g.scale.setScalar(lerp(1, 1.5, o));
      anchors[i].position.set(g.position.x, g.position.y + 0.45 * size * o, 0.1 * size);
      const on = highlight === -2 || highlight === i;
      mats[i].emissive.setHex(PALETTE.goldBright);
      mats[i].emissiveIntensity = on ? 0.8 * clamp01(l) : 0;
    }
  };
  grp.userData.segments = segs;
  grp.userData.fieldAnchors = anchors;
  grp.userData.fields = HEADER_FIELDS;
  grp.userData.setState();
  attachDispose(grp);
  return grp;
}

// ---- proof slab ---------------------------------------------------------------------------------
// Leaf (7 pins), node (5 pins), decider seal (4 pins). Same footprint for every variant.
const PIN_COUNT = { leaf: 7, node: 5, decider: 4 };

function makePin(kind, mat) {
  const g = new THREE.Group();
  g.userData.bodies = [];
  if (kind === 'collar' || kind === 'capcollar') {
    const c = new THREE.Mesh(sharedGeo('collar', () => new THREE.CylinderGeometry(0.17, 0.17, 0.3, 6)), mat);
    c.rotation.x = Math.PI / 2;
    g.add(c);
    g.userData.bodies.push(c);
    if (kind === 'capcollar') {
      const ring = new THREE.Mesh(sharedGeo('capring', () => new THREE.TorusGeometry(0.21, 0.03, 6, 20)), sharedMat('gold', () => materials.gold()));
      ring.position.z = 0.15;
      g.add(ring);
    }
  } else if (kind === 'bar') {
    const b = box(0.34, 0.06, 0.3, mat);
    g.add(b);
    g.userData.bodies.push(b);
  } else {
    const b = box(0.22, 0.22, 0.3, mat);
    g.add(b);
    g.userData.bodies.push(b);
  }
  return g;
}

// Pooled materials of a slab whose scene does not tint it on its own: pin colours (one per family and titanium),
// the lit edge in quarter steps and the closing arc (plain, lit). `own: true` gives the slab private materials
// instead (scenes 4 and 5 dim their slabs by rewriting material colours).
const lvl = (v) => Math.round(clamp01(v) * 4);
const edgeMatOf = (level) => sharedMat(`edge${level}`, () => { const m = materials.gold(); m.emissive.setHex(PALETTE.goldBright); m.emissiveIntensity = 0.8 * level / 4; return m; });
const pinMatOf = (fam) => sharedMat(`pin${fam || 'ti'}`, () => (fam ? materials.titanium({ roughness: 0.5, color: (FAMILY[fam] || FAMILY.state).hex }) : materials.titanium({ roughness: 0.5 })));

export function makeProofSlab({ variant = 'leaf', scale = 1, pinKinds = [], own = false } = {}) {
  const n = PIN_COUNT[variant] || 7;
  const grp = new THREE.Group();
  const edgeMat = own ? materials.gold() : edgeMatOf(0);
  const edge = box(3.14, 0.1, 2.14, edgeMat);
  edge.position.y = 0.05;
  const body = box(3.0, 0.3, 2.0, sharedMat('titanium', () => materials.titanium()));
  body.position.y = 0.25;
  grp.add(edge, body);

  const pins = [];
  const pinMats = [];
  const pinAnchors = [];
  for (let i = 0; i < n; i++) {
    const mat = own ? materials.titanium({ roughness: 0.5 }) : pinMatOf(null);
    const pin = makePin(pinKinds[i] || 'box', mat);
    pin.position.set(n > 1 ? -1.3 + (i * 2.6) / (n - 1) : 0, 0.25, 1.15);
    grp.add(pin);
    const a = new THREE.Object3D();
    a.position.set(pin.position.x, 0.25, 1.4);
    grp.add(a);
    pins.push(pin); pinMats.push(mat); pinAnchors.push(a);
  }

  // Accumulator ring: 330 degree arc, a 30 degree unclosed joint toward +z, and the closing arc.
  const ringMat = sharedMat('gold', () => materials.gold());
  const closeMat = own ? materials.gold() : sharedMat('gold', () => materials.gold());
  const closeLit = own ? null : edgeMatOf(4);
  const mkArc = (arcDeg, startDeg) => sharedGeo(`arc${arcDeg}|${startDeg}`, () => {
    const geo = new THREE.TorusGeometry(0.7, 0.08, 8, 48, rad(arcDeg));
    geo.rotateZ(rad(startDeg));
    geo.rotateX(-Math.PI / 2);
    return geo;
  });
  const ring = new THREE.Group();
  ring.position.y = 0.48;
  ring.add(new THREE.Mesh(mkArc(330, 285), ringMat));
  const closer = new THREE.Mesh(mkArc(30, 255), closeMat);
  ring.add(closer);
  const foldMats = ringMat;
  const folds = [];
  for (let k = 0; k < 12; k++) {
    const a = rad(285 + (k + 0.5) * (330 / 12));
    const m = box(0.1, 0.12, 0.2, foldMats);
    m.position.set(0.7 * Math.cos(a), 0.06, -0.7 * Math.sin(a));
    m.rotation.y = Math.atan2(-Math.sin(a), -Math.cos(a));
    ring.add(m);
    folds.push(m);
  }
  grp.add(ring);

  // Verification sweep bar.
  const sweepMat = sharedMat('sweep', () => {
    const m = materials.gold();
    m.emissive.setHex(PALETTE.goldBright);
    m.emissiveIntensity = 1;
    return m;
  });
  const sweep = box(0.08, 0.02, 2.0, sweepMat);
  sweep.position.y = 0.41;
  grp.add(sweep);

  // pins: array of null or a family name per pin (the family colour marks a pin the beat names).
  // verify 0..1: the bar crosses the top face. edgeLit 0..1: the gold edge glows (check passed).
  // ring: { show 0..1, folds: count 0..12, closed 0..1 } (node and decider carry the accumulator).
  grp.userData.setState = ({ pins: pc = [], verify = 0, edgeLit = 0, ring: rs = { show: 0, folds: 0, closed: 0 } } = {}) => {
    for (let i = 0; i < n; i++) {
      const f = pc[i];
      if (own) pinMats[i].color.setHex(f ? (FAMILY[f] || FAMILY.state).hex : PALETTE.titanium);
      else { const pm = pinMatOf(f || null); for (const m of pins[i].userData.bodies) m.material = pm; }
    }
    const v = clamp01(verify);
    sweep.visible = v > 0 && v < 1;
    sweep.position.x = lerp(-1.5, 1.5, v);
    if (own) {
      edgeMat.emissive.setHex(PALETTE.goldBright);
      edgeMat.emissiveIntensity = 0.8 * clamp01(edgeLit);
    } else edge.material = edgeMatOf(lvl(edgeLit));
    const show = clamp01(rs.show ?? 0);
    ring.visible = show > 0.001;
    ring.scale.setScalar(Math.max(show, 0.001));
    const nf = Math.max(0, Math.min(12, Math.floor(rs.folds ?? 0)));
    folds.forEach((m, k) => { m.visible = k < nf; });
    const c = clamp01(rs.closed ?? 0);
    closer.visible = c > 0.001;
    closer.scale.setScalar(Math.max(c, 0.001));
    if (own) {
      closeMat.emissive.setHex(PALETTE.goldBright);
      closeMat.emissiveIntensity = c > 0 && c < 1 ? 0.8 : 0;
    } else closer.material = c > 0 && c < 1 ? closeLit : closeMat;
  };
  grp.userData.variant = variant;
  grp.userData.pins = pins;
  grp.userData.pinAnchors = pinAnchors;
  grp.userData.ring = ring;
  grp.scale.setScalar(scale);
  grp.userData.setState();
  attachDispose(grp);
  return grp;
}

// ---- claim plate --------------------------------------------------------------------------------
// The drawer plate of the runs cabinet: a slate body in a gold rim, with claim_d at the centre and
// the stored outcome_d and n_steps in front.
export function makeClaimPlate({ claim = 'claim:main', outcome = 'outcome:main', steps = 'n_steps:main', size = 1 } = {}) {
  const grp = new THREE.Group();
  const rimMat = materials.gold();
  const rim = box(2.94, 0.1, 2.14, rimMat);
  rim.position.y = 0.05;
  const body = box(2.8, 0.14, 2.0, sharedMat('slate', () => materials.slate()));
  body.position.y = 0.17;
  grp.add(rim, body);
  const tokClaim = makeDigestToken({ family: 'state', content: claim, size: 0.9 });
  const tokOut = makeDigestToken({ family: 'outcome', content: outcome, size: 0.5 });
  const tokSteps = makeDigestToken({ family: 'state', content: steps, size: 0.5 });
  for (const [t, x, z] of [[tokClaim, 0, -0.3], [tokOut, -0.8, 0.6], [tokSteps, 0.8, 0.6]]) {
    t.rotation.x = -Math.PI / 2;
    t.position.set(x, 0.24 + 0.09 * (t === tokClaim ? 0.9 : 0.5) * 2, z);
    grp.add(t);
  }
  // claim 0..1 and stored 0..1 scale the tokens in from 0.2 (appear); lit 0..1 lights the rim.
  grp.userData.setState = ({ claim: c = 1, stored = 1, lit: l = 0 } = {}) => {
    const set = (t, a) => { t.visible = a > 0.001; t.scale.setScalar(a > 0.001 ? lerp(0.2, 1, clamp01(a)) : 0.001); };
    set(tokClaim, c); set(tokOut, stored); set(tokSteps, stored);
    rimMat.emissive.setHex(PALETTE.goldBright);
    rimMat.emissiveIntensity = 0.8 * clamp01(l);
  };
  grp.userData.claimToken = tokClaim;
  grp.userData.outcomeToken = tokOut;
  grp.userData.stepsToken = tokSteps;
  grp.scale.setScalar(size);
  grp.userData.setState();
  attachDispose(grp);
  return grp;
}

// ---- decider verifier key card ------------------------------------------------------------------
// The one key that is on-chain: a gold key card at scale 0.6 inside a titanium frame 0.12 thick, with one
// ink notch. The proving keys (leaf, node, decider) are plain gold key cards and never appear on-chain.
// setState({ lit }) lights the card (0..1). Footprint about 1.2 by 0.84, standing in the xy plane (face toward +z).
export function makeVerifierKeyCard({ size = 1 } = {}) {
  const grp = new THREE.Group();
  const cardMat = materials.gold();
  const CW = 1.6 * 0.6, CH = 1.0 * 0.6, F = 0.12;
  const card = box(CW, CH, 0.08, cardMat);
  grp.add(card);
  const ti = sharedMat('titanium', () => materials.titanium());
  for (const [w, h, x, y] of [[CW + 2 * F, F, 0, CH / 2 + F / 2], [CW + 2 * F, F, 0, -CH / 2 - F / 2], [F, CH, -CW / 2 - F / 2, 0], [F, CH, CW / 2 + F / 2, 0]]) {
    const b = box(w, h, 0.14, ti);
    b.position.set(x, y, 0);
    grp.add(b);
  }
  const notch = box(0.13, 0.1, 0.04, sharedMat('ink', () => materials.ink()));
  notch.position.set(0, -CH / 2 + 0.12, 0.05);
  grp.add(notch);
  const anchor = new THREE.Object3D();
  anchor.position.set(0, CH / 2 + F + 0.15, 0);
  grp.add(anchor);
  grp.userData.anchor = anchor;
  grp.userData.setState = ({ lit = 0 } = {}) => {
    cardMat.emissive.setHex(PALETTE.goldBright);
    cardMat.emissiveIntensity = 0.8 * clamp01(lit);
  };
  grp.userData.setState();
  grp.scale.setScalar(size);
  attachDispose(grp);
  return grp;
}
