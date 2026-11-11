// Larger fixed objects of scene 7: the envelope, the drawer, the cabinet, the deck, the tray.
import * as THREE from 'three/webgpu';

const PI = Math.PI;
export const SOCK_X = [-1.9, -0.9, 0.1, 1.2];

// Envelope from scene 6: 5.2 by 0.3 by 3.6 open case, four sockets on the front (+z) edge.
export function makeEnv(P, { kit }) {
  const { m, g, bx, grp } = P;
  const root = new THREE.Group();
  root.add(bx(m.slate, 5.2, 0.3, 3.6));
  root.add(bx(m.ti, 5.3, 0.08, 0.1, 0, 0.17, 1.75), bx(m.ti, 5.3, 0.08, 0.1, 0, 0.17, -1.75));
  root.add(bx(m.ti, 0.1, 0.08, 3.5, -2.6, 0.17, 0), bx(m.ti, 0.1, 0.08, 3.5, 2.6, 0.17, 0));
  root.add(bx(m.ink, 3.2, 0.02, 2.2, -0.7, 0.155, -0.3));
  for (let i = 0; i < 3; i++) {
    const h = new THREE.Mesh(g.hex, m.ink);
    h.scale.set(0.9, 0.02, 0.9);
    h.position.set(SOCK_X[i], 0.155, 1.3);
    root.add(h);
  }
  root.add(bx(m.ink, 0.7, 0.02, 0.5, SOCK_X[3], 0.155, 1.3));
  root.add(bx(m.gold, 2.4, 0.1, 0.5, 0, 0.2, -1.55));
  const seal = kit ? P.kitSlab('decider') : P.plainSlab({ pins: 4 });
  seal.position.set(-0.7, 0.15, -0.3);
  root.add(seal);
  const toks = [P.tok('program', 'prog:main'), P.tok('input', 'input:main'), P.tok('outcome', 'outcome:main')];
  toks.forEach((t, i) => {
    t.rotation.x = -PI / 2;
    t.scale.setScalar(0.8);
    t.position.set(SOCK_X[i], 0.232, 1.3);
    root.add(t);
  });
  const collar = P.collar();
  collar.position.set(SOCK_X[3], 0.32, 1.3);
  root.add(collar);
  const lid = grp(bx(m.slate, 5.2, 0.08, 2.2));
  root.add(lid);
  const hatch = grp();
  for (const k of [-0.9, 0, 0.9]) {
    const b = bx(m.err, 0.08, 0.02, 3.0);
    b.rotation.y = PI / 4;
    b.position.set(k * 0.5 * Math.SQRT1_2, 0, -k * 0.5 * Math.SQRT1_2);
    hatch.add(b);
  }
  hatch.position.set(-0.7, 0.62, -0.3);
  root.add(hatch);
  return { root, seal, toks, collar, lid, hatch };
}

// Drawer: origin at the centre of the front face (front panel spans z -0.15 to 0), body runs to -z.
export function makeDrawer(P, { struck = true } = {}) {
  const { m, g, bx } = P;
  const root = new THREE.Group();
  root.add(bx(m.ti, 3, 1, 0.15, 0, 0, -0.075));
  const hf = new THREE.Mesh(g.hex, m.ink);
  hf.rotation.x = PI / 2;
  hf.scale.set(0.9, 0.03, 0.9);
  root.add(hf);
  root.add(bx(m.pin, 3, 0.1, 1.85, 0, 0.25, -1.075));
  root.add(bx(m.ti, 0.1, 0.3, 1.85, -1.45, 0.45, -1.075), bx(m.ti, 0.1, 0.3, 1.85, 1.45, 0.45, -1.075));
  const hi = new THREE.Mesh(g.hex, m.ink);
  hi.scale.set(0.8, 0.02, 0.8);
  hi.position.set(-0.9, 0.31, -1.0);
  root.add(hi);
  root.add(bx(m.ink, 0.6, 0.02, 0.4, 0.1, 0.31, -1.0));
  if (struck) {
    const y = 0.31;
    root.add(bx(m.ink, 0.7, 0.02, 0.05, 1.0, y, -1.3), bx(m.ink, 0.7, 0.02, 0.05, 1.0, y, -0.7));
    root.add(bx(m.ink, 0.05, 0.02, 0.7, 0.65, y, -1.0), bx(m.ink, 0.05, 0.02, 0.7, 1.35, y, -1.0));
    const st = bx(m.ink, 0.06, 0.025, 1.0, 1.0, y, -1.0);
    st.rotation.y = PI / 4;
    root.add(st);
  }
  return root;
}

export const DRW_CLOSED = [3, 1.75, -3.5];
export const DRW_OUT = [3, 1.75, -1.7];
// Inner seat offsets in drawer-local space.
export const SEAT_OUT = [-0.9, 0.365, -1.0];
export const SEAT_COLLAR = [0.1, 0.48, -1.0];

export function makeCabinet(P) {
  const { m, bx, g } = P;
  const root = new THREE.Group();
  root.add(bx(m.slate, 9.6, 3.5, 2.0, 3, 1.75, -4.6));
  root.add(bx(m.ti, 9.8, 0.1, 2.2, 3, 3.55, -4.6));
  root.add(bx(m.ink, 2.9, 0.9, 0.05, 3, 1.75, -3.58));
  const fronts = new THREE.InstancedMesh(g.box, m.ti, 8);
  fronts.frustumCulled = false;
  const M4 = new THREE.Matrix4();
  let k = 0;
  for (let c = 0; c < 3; c++) {
    for (let r = 0; r < 3; r++) {
      if (c === 1 && r === 1) continue;
      M4.compose(new THREE.Vector3(c * 3, 0.75 + r, -3.55), new THREE.Quaternion(), new THREE.Vector3(2.9, 0.9, 0.1));
      fronts.setMatrixAt(k++, M4);
    }
  }
  root.add(fronts);
  for (const dx of [0, 0.12]) {
    const b = bx(m.line, 0.05, 1.8, 0.04, 7.62 + dx, 1.75, -3.52);
    b.rotation.z = -20 * PI / 180;
    root.add(b);
  }
  return root;
}

export function makeDeck(P) {
  const { m, bx } = P;
  const root = new THREE.Group();
  root.add(bx(m.slate, 7, 0.3, 5, -11, 0.15, -1));
  root.add(bx(m.ti, 7.1, 0.08, 0.1, -11, 0.31, 1.5), bx(m.ti, 7.1, 0.08, 0.1, -11, 0.31, -3.5));
  root.add(bx(m.ti, 0.1, 0.08, 5, -14.5, 0.31, -1), bx(m.ti, 0.1, 0.08, 5, -7.5, 0.31, -1));
  // contract entrance: two posts and a lintel
  root.add(bx(m.ti, 0.3, 2.2, 0.3, -14.0, 1.1, -3.0), bx(m.ti, 0.3, 2.2, 0.3, -14.0, 1.1, 1.2), bx(m.ti, 0.3, 0.3, 4.5, -14.0, 2.35, -0.9));
  // key post
  root.add(bx(m.ti, 0.12, 1.9, 0.12, -11, 1.25, -3.2));
  return root;
}

export function makeTray(P) {
  const { m, bx } = P;
  const root = new THREE.Group();
  root.add(bx(m.paper, 20, 0.1, 5.4));
  root.add(bx(m.rimOff, 20, 0.08, 0.1, 0, 0.09, -2.65), bx(m.rimOff, 20, 0.08, 0.1, 0, 0.09, 2.65));
  root.add(bx(m.rimOff, 0.1, 0.08, 5.4, -9.95, 0.09, 0), bx(m.rimOff, 0.1, 0.08, 5.4, 9.95, 0.09, 0));
  return root;
}
