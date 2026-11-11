// Scene 5 rig: tracks (pure functions of time), instanced box pools, props and connectors.
// Every prop owns a Group `node` (position, uniform scale) that seek sets from its track. Plain boxes are
// instances of one InstancedMesh per material, so the scene stays inside the draw-call budget.
import * as THREE from 'three/webgpu';
import { PALETTE, materials, makeDigestToken, easeInOut, clamp01 } from '../../theme.js';

// ---- tracks ---------------------------------------------------------------------------------------
// A track holds N numbers (x, y, z, scale for a prop; one number for a scalar). Segments are appended in
// time order; the value is the start value before a segment and the end value after it.
export class Track {
  constructor(init) {
    this.init = init.slice();
    this.last = init.slice();
    this.segs = [];
  }

  // Move from the last value through pts (arrays; missing entries keep the previous value).
  go(t0, t1, pts, lin = false) {
    const path = [this.last.slice()];
    let prev = this.last;
    for (const p of pts) {
      const q = prev.map((v, i) => (p[i] === undefined ? v : p[i]));
      path.push(q);
      prev = q;
    }
    this.segs.push({ t0, t1, path, lin, end: prev.slice() });
    this.last = prev.slice();
    return this;
  }

  // Set the value at time t (a cut).
  jump(t, p) {
    const q = this.last.map((v, i) => (p[i] === undefined ? v : p[i]));
    this.segs.push({ t0: t, t1: t, path: [q, q], lin: true, end: q.slice() });
    this.last = q.slice();
    return this;
  }

  at(t, out) {
    let cur = this.init;
    for (const s of this.segs) {
      if (t < s.t0) break;
      if (t >= s.t1) { cur = s.end; continue; }
      const k = (t - s.t0) / (s.t1 - s.t0);
      const u = s.lin ? k : easeInOut(k);
      const n = s.path.length - 1;
      const f = u * n;
      const i = Math.min(Math.floor(f), n - 1);
      const w = f - i;
      const a = s.path[i], b = s.path[i + 1];
      for (let j = 0; j < out.length; j++) out[j] = a[j] + (b[j] - a[j]) * w;
      return out;
    }
    for (let j = 0; j < out.length; j++) out[j] = cur[j];
    return out;
  }
}

// ---- pools ----------------------------------------------------------------------------------------
const POOL_MATS = {
  ceramic: () => materials.ceramic(),
  ink: () => materials.ink(),
  titanium: () => materials.titanium(),
  gold: () => materials.gold(),
  goldLit: () => materials.gold({ emissive: PALETTE.goldBright, emissiveIntensity: 0.8 }),
  sweep: () => materials.gold({ color: PALETTE.goldBright, emissive: PALETTE.goldBright, emissiveIntensity: 1 }),
  line: () => new THREE.MeshBasicMaterial({ color: PALETTE.onSurface }),
  program: () => materials.family('program'),
  input: () => materials.family('input'),
  outcome: () => materials.family('outcome'),
  step: () => materials.family('step'),
  error: () => materials.family('error', { lit: 0.6 }),
  paper: () => materials.paper(),
};

const _m = new THREE.Matrix4();
const _q = new THREE.Quaternion();
const _e = new THREE.Euler();
const _p = new THREE.Vector3();
const _s = new THREE.Vector3();
const _zero = new THREE.Matrix4().makeScale(0, 0, 0);
const _X = new THREE.Vector3(1, 0, 0);
const _d = new THREE.Vector3();
const _a = new THREE.Vector3();
const _b = new THREE.Vector3();

export class Rig {
  constructor(scene, cap = 260) {
    this.scene = scene;
    this.props = [];
    this.links = [];
    this.geo = new THREE.BoxGeometry(1, 1, 1);
    this.pools = {};
    for (const k of Object.keys(POOL_MATS)) {
      const mesh = new THREE.InstancedMesh(this.geo, POOL_MATS[k](), cap);
      mesh.frustumCulled = false;
      mesh.count = 0;
      scene.add(mesh);
      this.pools[k] = { mesh, n: 0, cap };
    }
    this.tmp = [0, 0, 0, 1];
    this.disposables = [];
  }

  alloc(pool, local) {
    const p = this.pools[pool];
    if (p.n >= p.cap) throw new Error('pool full ' + pool);
    p.mesh.setMatrixAt(p.n, _zero);
    return { pool: p, idx: p.n++, local };
  }

  prop(init = [0, 0, 0, 1]) {
    const pr = new Prop(this, init);
    this.props.push(pr);
    this.scene.add(pr.node);
    return pr;
  }

  link(a, b, grow, thick = 0.05) {
    const l = new Link(this, a, b, grow, thick);
    this.links.push(l);
    return l;
  }

  finalize() {
    for (const k of Object.keys(this.pools)) this.pools[k].mesh.count = this.pools[k].n;
  }

  // Evaluate every prop at time t, then write the instance matrices.
  update(t) {
    for (const pr of this.props) pr.evaluate(t);
    for (const pr of this.props) pr.flush();
    for (const l of this.links) l.flush(t);
    for (const k of Object.keys(this.pools)) {
      const p = this.pools[k];
      if (p.n) p.mesh.instanceMatrix.needsUpdate = true;
    }
  }

  dispose() {
    this.geo.dispose();
    for (const k of Object.keys(this.pools)) {
      this.pools[k].mesh.material.dispose();
      this.scene.remove(this.pools[k].mesh);
    }
    for (const d of this.disposables) d();
    this.props.length = 0;
    this.links.length = 0;
  }
}

export class Prop {
  constructor(rig, init) {
    this.rig = rig;
    this.node = new THREE.Group();
    this.tr = new Track(init);
    this.parts = [];
    this.sel = 0;
    this.off = false;
    this.node.position.set(init[0], init[1], init[2]);
    this.node.scale.setScalar(Math.max(init[3], 0.0001));
    this.node.visible = init[3] > 0.002;
  }

  // Add a box instance: pool name, local position, local size, optional euler rotation, optional selector.
  box(pool, x, y, z, w, h, d, rot, sel) {
    const local = new THREE.Matrix4();
    _e.set(rot ? rot[0] : 0, rot ? rot[1] : 0, rot ? rot[2] : 0);
    _q.setFromEuler(_e);
    local.compose(_p.set(x, y, z), _q, _s.set(w, h, d));
    const part = this.rig.alloc(pool, local);
    part.sel = sel;
    this.parts.push(part);
    return this;
  }

  // Gold outline frame in the xz plane (w by d outside size, thickness th); sel 0 plain, sel 1 lit.
  frame(w, d, th = 0.05, y = 0) {
    for (const [pool, sel] of [['gold', 0], ['goldLit', 1]]) {
      this.box(pool, 0, y, -d / 2, w + th, th, th, null, sel);
      this.box(pool, 0, y, d / 2, w + th, th, th, null, sel);
      this.box(pool, -w / 2, y, 0, th, th, d + th, null, sel);
      this.box(pool, w / 2, y, 0, th, th, d + th, null, sel);
    }
    return this;
  }

  evaluate(t) {
    this.tr.at(t, this.rig.tmp);
    const v = this.rig.tmp;
    this.node.position.set(v[0], v[1], v[2]);
    this.node.scale.setScalar(Math.max(v[3], 0.0001));
    this.node.visible = v[3] > 0.002 && !this.off;
  }

  flush() {
    if (!this.parts.length) return;
    this.node.updateMatrix();
    const show = this.node.visible;
    for (const pt of this.parts) {
      if (!show || (pt.sel !== undefined && pt.sel !== this.sel)) pt.pool.mesh.setMatrixAt(pt.idx, _zero);
      else { _m.multiplyMatrices(this.node.matrix, pt.local); pt.pool.mesh.setMatrixAt(pt.idx, _m); }
    }
  }

  // Child anchor for labels (offset in node-local units).
  anchor(x = 0, y = 0, z = 0) {
    const a = new THREE.Object3D();
    a.position.set(x, y, z);
    this.node.add(a);
    return a;
  }
}

// A connector: a thin box from a to b that grows from a. a and b are {p: Prop|null, o: [x,y,z]}.
export class Link {
  constructor(rig, a, b, grow, thick) {
    this.a = a; this.b = b; this.grow = grow; this.thick = thick;
    this.part = rig.alloc('line', null);
    this.rig = rig;
    this.g = [0];
  }

  static end(p, o) { return { p, o }; }

  flush(t) {
    this.grow.at(t, this.g);
    const g = this.g[0];
    const pool = this.part.pool.mesh;
    if (g < 0.002) { pool.setMatrixAt(this.part.idx, _zero); return; }
    _a.set(this.a.o[0], this.a.o[1], this.a.o[2]);
    if (this.a.p) _a.add(this.a.p.node.position);
    _b.set(this.b.o[0], this.b.o[1], this.b.o[2]);
    if (this.b.p) _b.add(this.b.p.node.position);
    _d.subVectors(_b, _a);
    const len = _d.length() * g;
    if (len < 1e-4) { pool.setMatrixAt(this.part.idx, _zero); return; }
    _d.normalize();
    _q.setFromUnitVectors(_X, _d);
    _p.copy(_a).addScaledVector(_d, len / 2);
    _m.compose(_p, _q, _s.set(len, this.thick, this.thick));
    pool.setMatrixAt(this.part.idx, _m);
  }
}

// ---- shared digest tokens ---------------------------------------------------------------------------
// makeDigestToken builds three materials and two geometries per token. Scene 5 draws about thirty tokens, so
// each family shares one rim material, all share one cap material and one body geometry, and each content
// string shares one face material. The extra objects that the helper creates are freed at once.
const shared = { rim: {}, cap: null, body: null, face: null, faceMat: {} };

export function shareToken(grp, family, content) {
  const body = grp.children[0];
  const face = grp.children[1];
  const [rim, cap1, cap2] = body.material;
  if (!shared.rim[family]) shared.rim[family] = rim; else rim.dispose();
  if (!shared.cap) shared.cap = cap1;
  else { cap1.dispose(); }
  cap2.dispose();
  if (!shared.body) shared.body = body.geometry; else body.geometry.dispose();
  if (!shared.face) shared.face = face.geometry; else face.geometry.dispose();
  if (!shared.faceMat[content]) shared.faceMat[content] = face.material; else face.material.dispose();
  body.material = [shared.rim[family], shared.cap, shared.cap];
  body.geometry = shared.body;
  face.geometry = shared.face;
  face.material = shared.faceMat[content];
  return grp;
}

export function makeTok(family, content) {
  return shareToken(makeDigestToken({ family, content, size: 1 }), family, content);
}

export function disposeShared() {
  for (const k of Object.keys(shared.rim)) shared.rim[k].dispose();
  if (shared.cap) shared.cap.dispose();
  if (shared.body) shared.body.dispose();
  if (shared.face) shared.face.dispose();
  for (const k of Object.keys(shared.faceMat)) shared.faceMat[k].dispose();
  shared.rim = {}; shared.cap = null; shared.body = null; shared.face = null; shared.faceMat = {};
}
