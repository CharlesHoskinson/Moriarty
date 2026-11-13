// Keyframe rig: every animated thing of scene 7 is an Item with a list of keys. The pose at scene
// time t is a pure function of the keys, so seek(t) keeps no state between calls.
import * as THREE from 'three/webgpu';
import { easeInOut } from '../../theme.js';

const NAMES = ['x', 'y', 'z', 's', 'lit', 'dim', 'rx', 'ry', 'rz', 'sl', 'v', 'e', 'rg', 'fo', 'g'];
const DEF = { x: 0, y: 0, z: 0, s: 0, lit: 0, dim: 0, rx: 0, ry: 0, rz: 0, sl: 1, v: 0, e: 0, rg: 0, fo: 0, g: 1 };

export class Item {
  constructor(tg) {
    this.tg = tg;
    this.keys = [];
    this.cur = { ...DEF };
  }

  push(t, lin = false) {
    const k = { t, lin };
    for (const n of NAMES) k[n] = this.cur[n];
    this.keys.push(k);
  }

  hold(t) {
    const l = this.keys[this.keys.length - 1];
    if (!l) return;
    if (l.t > t + 1e-6) throw new Error(`scene7 key order ${t} < ${l.t}`);
    if (l.t < t) this.push(t);
  }

  at(t, p) {
    this.hold(t);
    Object.assign(this.cur, p);
    this.push(t);
    return this;
  }

  to(t0, t1, p, lin = false) {
    this.hold(t0);
    Object.assign(this.cur, p);
    this.push(t1, lin);
    return this;
  }

  show(t, p = {}, dur = 0.4) {
    const s = p.s ?? 1;
    const l = this.keys[this.keys.length - 1];
    this.hold(l ? Math.max(l.t, t - 1e-3) : t);
    Object.assign(this.cur, p, { s: 0 });
    this.push(t);
    this.cur.s = s;
    this.push(t + dur);
    return this;
  }

  hide(t, dur = 0.4) {
    this.hold(t);
    this.cur.s = 0;
    this.push(t + dur);
    return this;
  }

  // Grow a connector from its source end.
  grow(t0, t1) {
    const l = this.keys[this.keys.length - 1];
    this.hold(l ? Math.max(l.t, t0 - 1e-3) : t0);
    Object.assign(this.cur, { s: 1, g: 0 });
    this.push(t0);
    this.cur.g = 1;
    this.push(t1);
    return this;
  }

  pulse(t, rise, fall, peak = 1, prop = 'lit') {
    this.to(t, t + rise, { [prop]: peak });
    this.to(t + rise, t + rise + fall, { [prop]: 0 });
    return this;
  }

  shift(t0, t1, dx, dy, dz) {
    return this.to(t0, t1, { x: this.cur.x + dx, y: this.cur.y + dy, z: this.cur.z + dz });
  }
}

const ZERO = new THREE.Matrix4().makeScale(0, 0, 0);
const M4 = new THREE.Matrix4();
const P3 = new THREE.Vector3();
const S3 = new THREE.Vector3();
const Q4 = new THREE.Quaternion();
const EU = new THREE.Euler();
const VA = new THREE.Vector3();
const VB = new THREE.Vector3();

export class ObjTarget {
  constructor(o) { this.o = o; }
  apply(st) {
    const o = this.o;
    if (st.s <= 0.002) { o.visible = false; return; }
    o.visible = true;
    o.position.set(st.x, st.y, st.z);
    o.rotation.set(st.rx, st.ry, st.rz);
    o.scale.set(st.s, st.s, st.s * st.sl);
    const fx = o.userData.fx;
    if (fx) fx(st);
  }
}

export class InstTarget {
  constructor(im, i) { this.im = im; this.i = i; }
  apply(st) {
    if (st.s <= 0.002) { this.im.setMatrixAt(this.i, ZERO); return; }
    EU.set(st.rx, st.ry, st.rz);
    Q4.setFromEuler(EU);
    M4.compose(P3.set(st.x, st.y, st.z), Q4, S3.set(st.s, st.s, st.s * st.sl));
    this.im.setMatrixAt(this.i, M4);
  }
}

// A connector between two points. An end is { p: [x,y,z] } or { o: Object3D, p: offset }.
export class ConnTarget {
  constructor(mesh, a, b, { w = 0.06, h = 0.04, mat, matLit, mid } = {}) {
    this.mesh = mesh; this.a = a; this.b = b; this.w = w; this.h = h;
    this.mat = mat; this.matLit = matLit; this.mid = mid;
  }
  pt(e, out) {
    if (e.o) out.copy(e.o.position).add(S3.set(e.p[0], e.p[1], e.p[2]));
    else out.set(e.p[0], e.p[1], e.p[2]);
  }
  apply(st) {
    const m = this.mesh;
    const gs = st.g * st.s;
    this.pt(this.a, VA);
    this.pt(this.b, VB);
    if (this.mid) this.mid.position.set((VA.x + VB.x) / 2, (VA.y + VB.y) / 2, (VA.z + VB.z) / 2);
    if (gs <= 0.002 || (this.a.o && this.a.o.visible === false) || (this.b.o && this.b.o.visible === false)) { m.visible = false; return; }
    m.visible = true;
    const dx = VB.x - VA.x, dy = VB.y - VA.y, dz = VB.z - VA.z;
    const L = Math.hypot(dx, dy, dz);
    m.position.set(VA.x + (dx * gs) / 2, VA.y + (dy * gs) / 2, VA.z + (dz * gs) / 2);
    m.rotation.set(0, Math.atan2(-dz, dx), Math.atan2(dy, Math.hypot(dx, dz)), 'YZX');
    m.scale.set(Math.max(L * gs, 1e-4), this.h, this.w);
    if (this.matLit) m.material = st.lit > 0.5 ? this.matLit : this.mat;
  }
}

export class Rig {
  constructor() {
    this.items = [];
    this.late = [];
    this.ims = new Set();
    this.st = { ...DEF };
  }

  add(tg) {
    const it = new Item(tg);
    this.items.push(it);
    if (tg.im) this.ims.add(tg.im);
    return it;
  }

  addLate(tg) {
    const it = new Item(tg);
    this.late.push(it);
    return it;
  }

  ev(it, t) {
    const k = it.keys;
    const n = k.length;
    const st = this.st;
    if (!n || t < k[0].t) { st.s = 0; it.tg.apply(st); return; }
    let a = k[n - 1];
    if (t >= a.t) {
      for (const nm of NAMES) st[nm] = a[nm];
    } else {
      let lo = 0, hi = n - 1;
      while (hi - lo > 1) {
        const mid = (lo + hi) >> 1;
        if (k[mid].t <= t) lo = mid; else hi = mid;
      }
      a = k[lo];
      const b = k[hi];
      let u = (t - a.t) / (b.t - a.t);
      if (!b.lin) u = easeInOut(u);
      for (const nm of NAMES) st[nm] = a[nm] + (b[nm] - a[nm]) * u;
    }
    it.tg.apply(st);
  }

  apply(t) {
    for (const it of this.items) this.ev(it, t);
    for (const it of this.late) this.ev(it, t);
    for (const im of this.ims) im.instanceMatrix.needsUpdate = true;
  }
}
