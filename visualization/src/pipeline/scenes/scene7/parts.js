// Shared geometry, materials and small object builders for scene 7. Every colour comes from theme.js.
import * as THREE from 'three/webgpu';
import { PALETTE, materials, makeDigestToken } from '../../theme.js';
import { makeProofSlab } from '../../kit.js';

const PI = Math.PI;
const FAMS = ['program', 'input', 'outcome', 'state'];

export function createParts() {
  const geos = [];
  const mats = [];
  const G = (x) => { geos.push(x); return x; };
  const M = (x) => { mats.push(x); return x; };

  const g = {
    box: G(new THREE.BoxGeometry(1, 1, 1)),
    hex: G(new THREE.CylinderGeometry(0.5, 0.5, 1, 6)),
    collar: G(new THREE.CylinderGeometry(0.17, 0.17, 0.3, 6)),
    ring: G(new THREE.TorusGeometry(0.56, 0.035, 6, 6)),
    sq: G(new THREE.TorusGeometry(0.7071, 0.03, 4, 4)),
  };
  g.collar.rotateX(PI / 2);
  g.sq.rotateZ(PI / 4);

  const m = {
    ti: M(materials.titanium()),
    pin: M(materials.titanium({ roughness: 0.5 })),
    gold: M(materials.gold()),
    ink: M(materials.ink()),
    cer: M(materials.ceramic()),
    cap: M(materials.ceramic({ roughness: 0.7 })),
    slate: M(materials.slate()),
    paper: M(materials.paper()),
    err: M(materials.family('error', { lit: 0.6 })),
    input: M(materials.family('input')),
    outcome: M(materials.family('outcome')),
    line: M(new THREE.MeshBasicMaterial({ color: PALETTE.onSurface })),
    lineLit: M(new THREE.MeshBasicMaterial({ color: PALETTE.goldBright })),
    sweep: M(materials.gold()),
  };
  m.sweep.emissive = new THREE.Color(PALETTE.goldBright);
  m.sweep.emissiveIntensity = 1;
  const rims = {};
  for (const f of FAMS) rims[f] = M(materials.family(f, { roughness: 0.45, metalness: 0.2 }));

  const bx = (mat, w, h, d, x = 0, y = 0, z = 0) => {
    const o = new THREE.Mesh(g.box, mat);
    o.scale.set(w, h, d);
    o.position.set(x, y, z);
    return o;
  };
  const grp = (...c) => { const o = new THREE.Group(); c.forEach((k) => o.add(k)); return o; };
  const ownGold = () => {
    const x = M(materials.gold());
    x.emissive = new THREE.Color(PALETTE.goldBright);
    x.emissiveIntensity = 0;
    return x;
  };
  const litFx = (o, mat) => { o.userData.fx = (st) => { mat.emissiveIntensity = st.lit; }; return o; };

  // ---- digest tokens at size 1 with shared geometry and shared materials; scale them with the rig
  const faceMats = new Map();
  let tokBody = null;
  let tokFace = null;
  function tok(family, content) {
    const t = makeDigestToken({ family, content, size: 1 });
    const [body, face] = t.children;
    if (!tokBody) { tokBody = G(body.geometry); tokFace = G(face.geometry); } else {
      body.geometry.dispose(); body.geometry = tokBody;
      face.geometry.dispose(); face.geometry = tokFace;
    }
    new Set(body.material).forEach((x) => x.dispose());
    body.material = [rims[family] || rims.state, m.cap, m.cap];
    const key = String(content);
    if (faceMats.has(key)) { face.material.dispose(); face.material = faceMats.get(key); } else {
      faceMats.set(key, face.material);
      mats.push(face.material);
    }
    return t;
  }

  // ---- proof slabs
  const matKey = new Map();
  const keyOf = (x) => `${x.color.getHex()}|${x.roughness}|${x.metalness}|${x.emissiveIntensity > 0 ? 1 : 0}`;
  for (const x of [m.ti, m.pin, m.gold, m.sweep]) matKey.set(keyOf(x), x);
  const geoKey = new Map();
  function kitSlab(variant) {
    const s = makeProofSlab({ variant });
    const edge = s.children[0];
    mats.push(edge.material);
    s.traverse((o) => {
      if (!o.isMesh) return;
      const gk = o.geometry.type + JSON.stringify(o.geometry.parameters);
      if (geoKey.has(gk)) { if (o.geometry !== geoKey.get(gk)) { o.geometry.dispose(); o.geometry = geoKey.get(gk); } } else { geoKey.set(gk, o.geometry); geos.push(o.geometry); }
      if (o === edge) return;
      const mk = keyOf(o.material);
      if (matKey.has(mk)) { if (o.material !== matKey.get(mk)) { o.material.dispose(); o.material = matKey.get(mk); } } else { matKey.set(mk, o.material); mats.push(o.material); }
    });
    s.userData.fx = (st) => s.userData.setState({ verify: st.v, edgeLit: st.e, ring: { show: st.rg, folds: st.fo, closed: 0 } });
    return s;
  }

  function plainSlab({ pins = 4, edge = 'gold' } = {}) {
    const own = edge === 'gold' ? ownGold() : null;
    const root = grp(bx(own || m.ti, 3.14, 0.1, 2.14, 0, 0.05, 0), bx(m.ti, 3, 0.3, 2, 0, 0.25, 0));
    for (let i = 0; i < pins; i++) root.add(bx(m.pin, 0.22, 0.22, 0.3, pins > 1 ? -1.3 + (i * 2.6) / (pins - 1) : 0, 0.25, 1.15));
    if (own) root.userData.fx = (st) => { own.emissiveIntensity = 0.8 * st.e; };
    return root;
  }

  function hash() {
    const own = ownGold();
    return litFx(grp(bx(own, 2.4, 1, 1.2), bx(m.ink, 0.1, 0.5, 0.6, -1.16, 0, 0), bx(m.ink, 0.1, 0.5, 0.6, 1.16, 0, 0)), own);
  }

  const collar = () => grp(new THREE.Mesh(g.collar, m.gold));

  // Key card lying flat: 1.6 by 0.08 by 1.0 with ink notches along the +z edge.
  function card(mat, notches, ownMat) {
    const root = grp(bx(ownMat || mat, 1.6, 0.08, 1.0));
    for (const nx of notches) root.add(bx(m.ink, 0.2, 0.09, 0.12, nx, 0, 0.44));
    if (ownMat) litFx(root, ownMat);
    return root;
  }

  const frameTi = (w, d, t = 0.06) => grp(
    bx(m.ti, w, 0.05, t, 0, 0, -d / 2), bx(m.ti, w, 0.05, t, 0, 0, d / 2),
    bx(m.ti, t, 0.05, d, -w / 2, 0, 0), bx(m.ti, t, 0.05, d, w / 2, 0, 0),
  );

  function ownMesh(geo, scale = 1) {
    const mat = ownGold();
    const o = litFx(grp(new THREE.Mesh(geo, mat)), mat);
    o.children[0].scale.setScalar(scale);
    return o;
  }

  // Outline of a box drawn with line segments.
  function outlineBox(w, h, d, x, y, z) {
    const eg = G(new THREE.EdgesGeometry(g.box));
    const ls = new THREE.LineSegments(eg, m.line);
    ls.scale.set(w, h, d);
    ls.position.set(x, y, z);
    return grp(ls);
  }

  function cellPool(n) {
    const white = new THREE.Color();
    white.setRGB(1, 1, 1);
    const im = new THREE.InstancedMesh(g.box, M(materials.ceramic({ color: white })), n);
    im.frustumCulled = false;
    const c = new THREE.Color(PALETTE.ceramic);
    for (let i = 0; i < n; i++) im.setColorAt(i, c);
    return im;
  }

  function platePool(n) {
    const im = new THREE.InstancedMesh(G(new THREE.BoxGeometry(1, 0.06, 1.2)), m.gold, n);
    im.frustumCulled = false;
    return im;
  }

  function dimFx(o, mat, toward) {
    const base = mat.color.clone();
    const tgt = new THREE.Color(toward);
    const prev = o.userData.fx;
    o.userData.fx = (st) => { mat.color.copy(base).lerp(tgt, 0.6 * st.dim); if (prev) prev(st); };
    return o;
  }

  function dispose() {
    geos.forEach((x) => x.dispose());
    mats.forEach((x) => x.dispose());
    geos.length = 0;
    mats.length = 0;
  }

  return { g, m, rims, bx, grp, ownGold, litFx, tok, kitSlab, plainSlab, hash, collar, card, frameTi, ownMesh, outlineBox, cellPool, platePool, dimFx, M, G, dispose };
}
