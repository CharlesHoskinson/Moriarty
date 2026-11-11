// Scene 4 helpers: shared geometry, materials, mini slabs, dimming and the connector pool.
// Everything here is created in build and freed by dispose; nothing is created inside seek.
import * as THREE from 'three/webgpu';
import { PALETTE, materials, clamp01 } from '../../theme.js';

const rad = (d) => (d * Math.PI) / 180;

export class Parts {
  constructor(scene) {
    this.scene = scene;
    this.geos = [];
    this.mats = [];
    this.UB = this.geo(new THREE.BoxGeometry(1, 1, 1));
    this.X = new THREE.Vector3(1, 0, 0);
    this.v = new THREE.Vector3();
    this.dimColor = new THREE.Color(PALETTE.paperWarm);
    this.tmpColor = new THREE.Color();
  }

  geo(g) { this.geos.push(g); return g; }
  mat(m) { this.mats.push(m); return m; }

  // A unit box scaled to w by h by d. Sizes live in the scale so one geometry serves all.
  box(mat, w, h, d, x = 0, y = 0, z = 0, parent = this.scene) {
    const m = new THREE.Mesh(this.UB, mat);
    m.scale.set(w, h, d);
    m.position.set(x, y, z);
    parent.add(m);
    return m;
  }

  // Unlit line colour for connectors and ticks.
  lineMat() { return this.mat(new THREE.MeshBasicMaterial({ color: PALETTE.onSurface })); }

  // Gold material whose emissive is driven by the scene.
  goldMat() { return this.mat(materials.gold()); }

  setBox(m, w, h, d, x, y, z, s = 1) {
    const on = s > 0.001;
    m.visible = on;
    if (!on) return;
    m.scale.set(w * s, h * s, d * s);
    m.position.set(x, y, z);
  }

  // ---- connector pool: thin boxes between two points, growing from the first point.
  makeConnectors(n, mat) {
    this.conns = [];
    for (let i = 0; i < n; i++) {
      const m = new THREE.Mesh(this.UB, mat);
      m.visible = false;
      this.scene.add(m);
      this.conns.push(m);
    }
    this.ci = 0;
  }

  beginConnectors() { this.ci = 0; }

  conn(ax, ay, az, bx, by, bz, k = 1, w = 0.05) {
    const c = this.conns[this.ci++];
    if (!c) return;
    const kk = clamp01(k);
    const v = this.v.set(bx - ax, by - ay, bz - az);
    const full = v.length();
    const len = full * kk;
    if (kk <= 0.001 || full < 1e-4) { c.visible = false; return; }
    v.multiplyScalar(1 / full);
    c.visible = true;
    c.quaternion.setFromUnitVectors(this.X, v);
    c.position.set(ax + v.x * len * 0.5, ay + v.y * len * 0.5, az + v.z * len * 0.5);
    c.scale.set(len, w, w);
  }

  endConnectors() {
    for (let i = this.ci; i < this.conns.length; i++) this.conns[i].visible = false;
  }

  // ---- dimming: lerp every colour of a group toward the ground by 0.6 * k. `live` materials
  // are rewritten by setState each frame, so the dimmer reads their current colour as the base.
  makeDimmer(root, live = []) {
    const list = [];
    const seen = new Set();
    const own = new Map();   // shared kit materials are cloned once per dimmed group, so dimming stays local
    const mine = (m) => {
      if (!m.userData.shared) return m;
      if (!own.has(m)) { const c = m.clone(); c.userData = { ...c.userData, shared: false }; this.mats.push(c); own.set(m, c); }
      return own.get(m);
    };
    root.traverse((o) => {
      if (!o.material) return;
      if (Array.isArray(o.material)) o.material = o.material.map(mine); else o.material = mine(o.material);
      for (const m of Array.isArray(o.material) ? o.material : [o.material]) {
        if (seen.has(m) || !m.color) continue;
        seen.add(m);
        list.push({ m, base: m.color.clone(), live: live.includes(m) });
      }
    });
    const dim = this.dimColor;
    const tmp = this.tmpColor;
    return (k) => {
      for (const e of list) {
        if (e.live) { tmp.copy(e.m.color).lerp(dim, 0.6 * k); e.m.color.copy(tmp); }
        else e.m.color.copy(e.base).lerp(dim, 0.6 * k);
      }
    };
  }

  // ---- mini slab: a proof slab drawn at small scale, with an optional ring and up to two marks.
  initMini() {
    const arc = this.geo(new THREE.TorusGeometry(0.7, 0.08, 6, 36, rad(330)));
    arc.rotateZ(rad(285));
    arc.rotateX(-Math.PI / 2);
    this.miniArc = arc;
    this.miniBody = this.mat(materials.titanium());
    this.miniPins = this.mat(materials.ceramic());
    this.miniGold = this.mat(materials.gold());
  }

  makeMini(edgeMat) {
    const g = new THREE.Group();
    this.box(edgeMat || this.miniGold, 3.14, 0.1, 2.14, 0, 0.05, 0, g);
    this.box(this.miniBody, 3.0, 0.3, 2.0, 0, 0.25, 0, g);
    this.box(this.miniPins, 2.6, 0.2, 0.3, 0, 0.25, 1.15, g);
    const ring = new THREE.Mesh(this.miniArc, this.miniGold);
    ring.position.y = 0.48;
    g.add(ring);
    const marks = [];
    for (let k = 0; k < 2; k++) {
      const a = rad(285 + (k + 0.5) * 27.5);
      const m = this.box(this.miniGold, 0.1, 0.12, 0.2, 0.7 * Math.cos(a), 0.54, -0.7 * Math.sin(a), g);
      m.rotation.y = Math.atan2(-Math.sin(a), -Math.cos(a));
      marks.push(m);
    }
    g.userData.ring = ring;
    g.userData.marks = marks;
    this.scene.add(g);
    return g;
  }

  putMini(g, x, y, z, s, ring = 0, marks = 0) {
    const on = s > 0.001;
    g.visible = on;
    if (!on) return;
    g.position.set(x, y, z);
    g.scale.setScalar(s);
    g.userData.ring.visible = ring > 0;
    g.userData.marks.forEach((m, k) => { m.visible = ring > 0 && k < marks; });
  }

  disposeAll() {
    for (const g of this.geos) g.dispose();
    for (const m of this.mats) m.dispose();
    this.geos.length = 0;
    this.mats.length = 0;
  }
}

export function disposeGroup(root, skipMaps = true) {
  root.traverse((o) => {
    if (o.geometry) o.geometry.dispose();
    if (o.material) {
      for (const m of Array.isArray(o.material) ? o.material : [o.material]) {
        if (!skipMaps && m.map) m.map.dispose();
        m.dispose();
      }
    }
  });
}
