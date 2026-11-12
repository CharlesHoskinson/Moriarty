// Scene 7 choreography: creates every object once and writes its keys (pure function of scene time).
import * as THREE from 'three/webgpu';
import { PALETTE } from '../../theme.js';
import { makeVerifierKeyCard } from '../../kit.js';
import { ObjTarget, InstTarget, ConnTarget } from './rig.js';
import { makeEnv, makeDrawer, makeCabinet, makeDeck, makeTray, DRW_CLOSED, DRW_OUT, SEAT_OUT, SEAT_COLLAR, SOCK_X } from './builders.js';

const PI = Math.PI;
const FLAT = -PI / 2;
export const DUR = [33, 27, 21, 29, 22, 22, 24, 29, 26, 31, 24, 26, 25, 25, 25, 22, 26, 28, 26, 29, 25];
export const T0 = DUR.reduce((a, d, i) => (a.push(i ? a[i - 1] + DUR[i - 1] : 0), a), []);
const B = (i, s) => T0[i] + s;

// tray coordinates (u, v) to scene coordinates
const X = (u) => 5 + u;
const Z = (v) => 3 + v;
const yT = (sz) => 0.16 + 0.09 * sz;   // centre height of a flat token of size sz on the tray
const CELL_Y = 0.35;

export function buildScript(ctx, P, rig) {
  const { m, g, bx, grp } = P;
  const sc = ctx.scene;
  const A = {};          // named anchors for labels
  const tray = [];       // items that the next tray beat clears
  const add = (o, parent = sc) => { parent.add(o); return rig.add(new ObjTarget(o)); };
  const clear = (t, keep = []) => { for (const it of tray) if (it.cur.s > 0 && !keep.includes(it)) it.hide(t); };
  const tok = (fam, content, onTray = true) => {
    const o = P.tok(fam, content);
    const it = add(o);
    if (onTray) tray.push(it);
    return { o, it };
  };
  const prim = (o, onTray = true, parent = sc) => { const it = add(o, parent); if (onTray) tray.push(it); return { o, it }; };

  // ---- instanced cells and gold edge plates
  const cells = P.cellPool(80);
  sc.add(cells);
  let ci = 0;
  const COL = new THREE.Color();
  const cell = (col = PALETTE.ceramic) => {
    const i = ci++;
    cells.setColorAt(i, COL.setHex(col));
    const it = rig.add(new InstTarget(cells, i));
    tray.push(it);
    return it;
  };
  const plates = P.platePool(12);
  sc.add(plates);
  let pi = 0;
  const plate = () => { const it = rig.add(new InstTarget(plates, pi++)); tray.push(it); return it; };

  // ---- connectors
  const connList = [];
  function conn(a, b, { gold = false, name } = {}) {
    const mesh = new THREE.Mesh(g.box, gold ? m.gold : m.line);
    sc.add(mesh);
    const mid = new THREE.Object3D();
    sc.add(mid);
    const tg = new ConnTarget(mesh, a, b, { w: gold ? 0.1 : 0.06, h: gold ? 0.1 : 0.04, mat: gold ? m.gold : m.line, matLit: gold ? m.gold : m.lineLit, mid });
    const it = rig.addLate(tg);
    tray.push(it);
    connList.push(it);
    if (name) A[name] = mid;
    return it;
  }
  const E = (e, p = [0, 0, 0]) => ({ o: e.o, p });
  const PT = (x, y, z) => ({ p: [x, y, z] });

  // ---- outline rings (hex) and frames (square) with their own lit material
  function outline(geo) {
    const mat = P.ownGold();
    const o = P.litFx(grp(new THREE.Mesh(geo, mat)), mat);
    return prim(o);
  }
  const rgs = [outline(g.ring), outline(g.ring), outline(g.ring), outline(g.ring), outline(g.ring)];
  const pairGeo = P.G(g.sq.clone().scale(0.7, 1.5, 1));
  const cardGeo = P.G(g.sq.clone().scale(1.9, 1.25, 1));
  const pairs = Array.from({ length: 6 }, () => outline(pairGeo));
  const cardRing = outline(cardGeo);
  // equality finish: the moving object hides, the outline shows with lit 0.5 for 0.3 s.
  const finishEq = (ring, t, p) => {
    ring.it.show(t, { ...p, rx: FLAT, lit: 0.5 }, 0.2);
    ring.it.to(t + 0.3, t + 0.45, { lit: 0 });
  };

  // ---- sweep bar
  const sw = prim(grp(bx(m.sweep, 0.08, 0.04, 1)), false);
  const sweep = (t0, t1, x0, x1, zc, len, y = 0.62) => {
    sw.it.show(t0, { x: x0, y, z: zc, sl: len, s: 1 }, 0.02);
    sw.it.to(t0 + 0.02, t1, { x: x1 }, true);
    sw.it.hide(t1, 0.02);
  };

  // ---------------------------------------------------------------- static set
  sc.add(makeDeck(P));
  const cab = makeCabinet(P);
  sc.add(cab);
  A.cab = new THREE.Object3D();
  A.cab.position.set(3, 3.6, -4.6);
  sc.add(A.cab);
  sc.add(P.bx(m.ti, 9.8, 0.06, 0.1, 2.7, 0.7, -3.2));
  [['claim:a', 0.75], ['claim:b', 1.75], ['claim:c', 2.75]].forEach(([c, y]) => {
    const t = P.tok('state', c);
    t.scale.setScalar(0.7);
    t.position.set(0, y, -3.5 + 0.07);
    sc.add(t);
  });

  // decider verifier key card (kit, AMEND-1 section 1.4), tilted so a top view reads it. The frame and
  // notch take the scene's titanium and ink; the card keeps its own gold for the lit pulse.
  const vkCard = makeVerifierKeyCard();
  const kitTi = vkCard.children[1].material;
  vkCard.traverse((o) => {
    if (!o.isMesh) return;
    if (o.material.userData.shared) o.material = o.material === kitTi ? m.ti : m.ink;
    else P.M(o.material);   // geometry stays in the kit cache
  });
  const vkSt = { lit: 0 };
  vkCard.userData.fx = (st) => { vkSt.lit = st.lit; vkCard.userData.setState(vkSt); };
  const vk = prim(vkCard, false);
  vk.it.at(0, { x: -11, y: 2.0, z: -3.0, s: 1.8, rx: 0.57 });
  vk.it.pulse(B(0, 11.0), 0.4, 0.5, 0.9);
  A.vk = vk.o;

  // ---------------------------------------------------------------- b0 envelope
  const ENV = [-11, 0.45, -1];
  const env = makeEnv(P, { kit: true });
  env.hatch.visible = false;
  const envI = prim(env.root, false);
  envI.it.at(0, { x: -20, y: 0.35, z: 0, s: 1 }).to(B(0, 2.0), B(0, 4.0), { x: ENV[0], y: ENV[1], z: ENV[2] });
  const lid = prim(env.lid, false, env.root);
  lid.it.at(0, { x: 0, y: 0.34, z: -0.3, s: 1 }).to(B(0, 4.4), B(0, 4.8), { y: 1.0 }).to(B(0, 4.8), B(0, 5.2), { y: 1.7, s: 0 });
  const seal = prim(env.seal, false, env.root);
  seal.it.at(0, { x: -0.7, y: 0.15, z: -0.3, s: 1 });
  seal.it.to(B(0, 11.15), B(0, 11.45), { e: 1 }).to(B(0, 11.8), B(0, 12.3), { e: 0 });
  A.seal = env.seal;
  A.envStrip = new THREE.Object3D();
  A.envStrip.position.set(-0.4, 0.2, 1.3);
  env.root.add(A.envStrip);
  const sw0 = B(0, 11.0);
  sweep(sw0, sw0 + 0.8, ENV[0] - 2.7, ENV[0] + 2.7, ENV[2], 3.8);
  A.collarEnv = env.collar;
  const proofA = tok('state', 'proof:cutA', false);
  env.root.add(proofA.o);
  proofA.it.show(B(5, 0.3), { x: -0.3, y: 0.64, z: -0.3, s: 0.5, rx: FLAT });

  // ---------------------------------------------------------------- b1 claim hash
  const hashC = prim(P.hash(), false);
  hashC.it.at(0, { x: -4.6, y: 0.5, z: -1.4, s: 1 });
  A.hashC = hashC.o;
  const dom = prim(grp(bx(m.gold, 0.4, 0.4, 0.4)), false);
  const ver = prim(grp(bx(m.cer, 0.4, 0.4, 0.4), bx(m.ink, 0.1, 0.02, 0.1, 0, 0.21, 0)), false);
  const cP = tok('program', 'prog:main', false);
  const cI = tok('input', 'input:main', false);
  const cO = tok('outcome', 'outcome:main', false);
  const claim = tok('state', 'claim:main', false);
  A.dom = dom.o; A.ver = ver.o; A.claim = claim.o; A.cO = cO.o;
  const outl = prim(P.outlineBox(0.5, 0.4, 0.5, 0, 0, 0), false);
  env.root.add(outl.o);
  outl.it.show(B(1, 2.0), { x: SOCK_X[3], y: 0.32, z: 1.3, s: 1 }, 0.3);
  A.outl = outl.o;
  {
    const h0 = B(1, 2.0);
    const fx = [-6.6, -7.25, -7.9, -8.55, -9.2];
    dom.it.show(h0 - 0.4, { x: fx[0], y: 0.95, z: -1.4, s: 1 }, 0.3);
    ver.it.show(h0 - 0.4, { x: fx[1], y: 0.95, z: -1.4, s: 1 }, 0.3);
    [cP, cI, cO].forEach((c, i) => {
      c.it.show(h0 - 0.4, { x: ENV[0] + SOCK_X[i], y: 0.68, z: 0.3, s: 0.8, rx: FLAT }, 0.3);
      c.it.to(h0, h0 + 0.6, { x: fx[2 + i], y: 0.95, z: -1.4, s: 0.55 });
    });
    [dom, ver, cP, cI, cO].forEach((c, i) => {
      const t = h0 + 0.6 + 0.24 * i;
      c.it.to(t, t + 0.24, { x: -5.8, y: 0.5, z: -1.4, s: 0 });
    });
    hashC.it.to(h0 + 1.8, h0 + 2.0, { lit: 0.8 }).to(h0 + 2.0, h0 + 2.2, { lit: 0 });
    claim.it.at(h0 + 2.2, { x: -3.4, y: 0.5, z: -1.4, s: 0.2, rx: FLAT });
    claim.it.to(h0 + 2.2, h0 + 3.0, { x: -2.2, y: 0.7, s: 1.2 });
  }

  // ---------------------------------------------------------------- b2 lookup
  claim.it.to(B(2, 2.0), B(2, 2.4), { x: -1.1, y: 0.75, z: -3.0, s: 0.9, rx: 0 });
  claim.it.to(B(2, 2.7), B(2, 3.0), { y: 1.75 }).to(B(2, 3.3), B(2, 3.6), { y: 2.75 });
  claim.it.to(B(2, 3.9), B(2, 4.4), { x: 3, y: 1.75, z: -2.9 });
  hashC.it.hide(B(3, 0));

  // ---------------------------------------------------------------- b3 store
  const drw = prim(makeDrawer(P, { struck: true }), false);
  drw.it.at(0, { x: DRW_CLOSED[0], y: DRW_CLOSED[1], z: DRW_CLOSED[2], s: 1 });
  drw.it.to(B(3, 2.0), B(3, 2.8), { z: DRW_OUT[2] });
  claim.it.to(B(3, 2.0), B(3, 2.8), { z: DRW_OUT[2] + 0.11, s: 0.8 });
  const D = DRW_OUT;
  const seat = (s) => [D[0] + s[0], D[1] + s[1], D[2] + s[2]];
  const collarCp = prim(P.collar(), false);
  A.collarCp = collarCp.o;
  cO.it.show(B(3, 3.2), { x: -0.3, y: 1.3, z: -1.0, s: 0.6, rx: FLAT }, 0.3);
  cO.it.to(B(3, 3.5), B(3, 4.5), { x: seat(SEAT_OUT)[0], y: seat(SEAT_OUT)[1], z: seat(SEAT_OUT)[2] });
  collarCp.it.show(B(3, 3.5), { x: -0.3, y: 1.3, z: -1.4, s: 1 }, 0.3);
  collarCp.it.to(B(3, 3.8), B(3, 4.8), { x: seat(SEAT_COLLAR)[0], y: seat(SEAT_COLLAR)[1], z: seat(SEAT_COLLAR)[2] });

  // ---------------------------------------------------------------- b4, b5 refusals
  const E2 = [-2.2, 0.15, -2.3];
  const S2 = 0.45;
  const env2 = makeEnv(P, { kit: false });
  env2.lid.visible = false;
  const env2I = prim(env2.root, false);
  const hatch2 = prim(env2.hatch, false, env2.root);
  hatch2.it.at(0, { x: -0.7, y: 0.62, z: -0.3, s: 0 });
  A.env2 = env2.root; A.seal2 = env2.seal; A.hatch2 = env2.hatch;
  const claim2 = tok('state', 'claim:main', false);
  A.claim2 = claim2.o;
  const proofB = tok('state', 'proof:cutB', false);
  env2.root.add(proofB.o);
  A.proofB = proofB.o;
  proofB.it.show(B(5, 0.6), { x: -0.3, y: 0.64, z: -0.3, s: 0.5, rx: FLAT });
  // tape board with two strips of 8 frames and cut markers
  const tape = new THREE.Group();
  tape.add(bx(m.ti, 3.2, 1.4, 0.08));
  const frames = new THREE.InstancedMesh(g.box, m.cer, 16);
  frames.frustumCulled = false;
  {
    const M4 = new THREE.Matrix4();
    for (let r = 0; r < 2; r++) for (let i = 0; i < 8; i++) {
      M4.compose(new THREE.Vector3((i - 3.5) * 0.3, r ? -0.25 : 0.25, 0.06), new THREE.Quaternion(), new THREE.Vector3(0.27, 0.36, 0.03));
      frames.setMatrixAt(r * 8 + i, M4);
    }
    tape.add(frames);
    for (const [k, r] of [[2, 0], [5, 0], [3, 1], [6, 1]]) tape.add(bx(m.ink, 0.03, 0.46, 0.04, (k - 3.5) * 0.3 + 0.15, r ? -0.25 : 0.25, 0.09));
  }
  const tapeI = prim(tape, false);
  A.tape = tape;
  const refuse = (b) => {
    const c = claim2.it;
    c.show(B(b, 0.6), { x: E2[0] - 0.4, y: 1.3, z: E2[2] + 0.1, s: 0.8, rx: 0 }, 0.3);
    c.to(B(b, 2.0), B(b, 2.9), { x: 3, y: 1.75, z: -0.3 });
    c.to(B(b, 2.9), B(b, 3.1), { z: -1.1 });
    c.to(B(b, 3.1), B(b, 3.4), { z: -0.7 });
    hatch2.it.show(B(b, 3.4), {}, 0.6);
  };
  env2I.it.show(B(4, 0.2), { x: E2[0], y: E2[1], z: E2[2], s: S2 }, 0.4);
  refuse(4);
  hatch2.it.hide(B(5, 0), 0.3);
  env2I.it.hide(B(5, 0), 0.4);
  claim2.it.hide(B(5, 0), 0.3);
  env2I.it.show(B(5, 0.6), { x: E2[0], y: E2[1], z: E2[2], s: S2 }, 0.4);
  refuse(5);
  tapeI.it.show(B(5, 0.6), { x: -2.0, y: 0.9, z: -3.42, s: 1 }, 0.4);
  hatch2.it.hide(B(6, 0), 0.3);
  env2I.it.hide(B(6, 0), 0.4);
  claim2.it.hide(B(6, 0), 0.3);
  tapeI.it.hide(B(6, 0), 0.4);

  // ---------------------------------------------------------------- tray
  const trayO = prim(makeTray(P), false);
  trayO.it.show(B(6, 0.2), { x: 5, y: -0.4, z: 3, s: 1 }, 0.2);
  trayO.it.to(B(6, 0.4), B(6, 1.0), { y: 0.1 });
  trayO.it.hide(B(20, 0), 0.6);
  A.tray = trayO.o;

  // ---- b6 collection slab, children, path to a claim
  const nodeA = prim(P.kitSlab('node'));
  A.nodeA = nodeA.o;
  const minis = [1.8, 3, 4.2].map((z) => prim(P.plainSlab({ pins: 0 })));
  const pnMat = [0, 1, 2].map(() => P.ownGold());
  const pn = pnMat.map((mt) => prim(P.litFx(grp(bx(mt, 0.4, 0.25, 0.4)), mt)));
  const sibs = [0, 1, 2].map((i) => tok('state', 'sib:' + i));
  const claimT = tok('state', 'claim:main');
  A.claimT = claimT.o; A.pn1 = pn[1].o; A.pn0 = pn[0].o;
  const pathC = [];
  const T6 = (s) => B(6, s);
  {
    nodeA.it.show(T6(0.8), { x: X(-5), y: 0.15, z: Z(0), s: 1, v: 0, e: 0, rg: 0, fo: 0 }, 0.4);
    minis.forEach((mi, i) => {
      mi.it.show(T6(0.8), { x: X(-7.5), y: 0.15, z: Z([-1.2, 0, 1.2][i]), s: 0.35 }, 0.4);
      conn(E(mi, [0.5, 0.1, 0]), PT(X(-6.5), 0.25, Z(Math.max(-0.9, Math.min(0.9, [-1.2, 0, 1.2][i])))), {}).grow(T6(1.2), T6(1.8));
    });
    const nv = [-0.4, 0.3, 1.0];
    pn.forEach((p, i) => p.it.show(T6(1.0), { x: X(-3), y: 0.275, z: Z(nv[i]), s: 1 }, 0.4));
    sibs.forEach((s, i) => s.it.show(T6(1.0), { x: X(-2.3), y: yT(0.4), z: Z(nv[i]), s: 0.4, rx: FLAT }, 0.4));
    claimT.it.show(T6(1.0), { x: X(-3), y: yT(0.8), z: Z(1.8), s: 0.8, rx: FLAT }, 0.4);
    pathC.push(conn(PT(X(-3.5), 0.25, Z(0)), E(pn[0]), {}).grow(T6(1.4), T6(1.9)));
    pathC.push(conn(E(pn[0]), E(pn[1])).grow(T6(1.4), T6(1.9)));
    pathC.push(conn(E(pn[1]), E(pn[2])).grow(T6(1.4), T6(1.9)));
    pathC.push(conn(E(pn[2]), E(claimT)).grow(T6(1.4), T6(1.9)));
    for (let i = 0; i < 3; i++) pathC.push(conn(E(pn[i]), E(sibs[i])).grow(T6(1.4), T6(1.9)));
    nodeA.it.to(T6(2.0), T6(2.4), { v: 1 }, true).to(T6(2.4), T6(2.41), { v: 0, e: 1 }).to(T6(2.7), T6(3.0), { e: 0 });
  }

  // ---- b7 open a sealed dependency
  const hashS = prim(P.hash());
  const setRoot = tok('state', 'set:prod');
  A.hashS = hashS.o; A.setRoot = setRoot.o;
  const ncopy = cell(PALETTE.gold);
  const scopy = tok('state', 'sib:2');
  const outT = tok('state', 'node:mid');
  {
    const t = (s) => B(7, s);
    const nv = [-1.4, -0.2, 1.0];
    pn.forEach((p, i) => p.it.to(t(0.2), t(1.5), { x: X(-2.5), z: Z(nv[i]), s: 1.3 }));
    sibs.forEach((s, i) => s.it.to(t(0.2), t(1.5), { x: X(-1.8), z: Z(nv[i]), s: 0.5, y: yT(0.5) }));
    claimT.it.to(t(0.2), t(1.5), { x: X(-2.5), z: Z(2.0) });
    setRoot.it.show(t(0.6), { x: X(-5), y: 0.63, z: Z(0), s: 0.8, rx: FLAT }, 0.4);
    hashS.it.show(t(0.6), { x: X(0.4), y: 0.4, z: Z(-0.2), s: 0.5 }, 0.4);
    const h = t(2.0);
    ncopy.show(h - 0.4, { x: X(-2.5), y: CELL_Y, z: Z(1.0), s: 0.4 }, 0.3);
    scopy.it.show(h - 0.4, { x: X(-1.8), y: yT(0.5), z: Z(1.0), s: 0.5, rx: FLAT }, 0.3);
    ncopy.to(h, h + 0.6, { x: X(-0.55), y: 0.5, z: Z(-0.2), s: 0.3 });
    scopy.it.to(h, h + 0.6, { x: X(-1.05), y: 0.5, z: Z(-0.2), s: 0.35 });
    ncopy.to(h + 0.6, h + 0.9, { x: X(-0.2), y: 0.4, z: Z(-0.2), s: 0 });
    scopy.it.to(h + 0.9, h + 1.2, { x: X(-0.2), y: 0.4, z: Z(-0.2), s: 0 });
    hashS.it.to(h + 1.8, h + 2.0, { lit: 0.8 }).to(h + 2.0, h + 2.2, { lit: 0 });
    outT.it.at(h + 2.2, { x: X(1.0), y: 0.5, z: Z(-0.2), s: 0.2, rx: FLAT });
    outT.it.to(h + 2.2, h + 3.0, { x: X(1.8), s: 0.5 });
    outT.it.to(h + 3.0, h + 3.4, { x: X(0.4), y: 1.0 }).to(h + 3.4, h + 3.7, { x: X(-1.6), y: 0.5 }).to(h + 3.7, h + 4.0, { x: X(-2.5), y: 0.35 });
    outT.it.hide(h + 4.0, 0.2);
    finishEq(rgs[0], h + 4.0, { x: X(-2.5), y: 0.35, z: Z(-0.2), s: 0.7 });
    pn[2].it.to(t(6.4), t(6.7), { lit: 0.7 });
    pn[0].it.to(t(6.8), t(7.1), { lit: 0.7 });
    nodeA.it.to(t(7.2), t(7.5), { e: 1 });
    // ---- b8 header fields against the dependency chain
    const q = (s) => B(8, s);
    clear(q(0), [...pn.map((x) => x.it), ...sibs.map((x) => x.it), claimT.it, ...pathC.slice(1)]);
    pn.forEach((p) => p.it.to(q(0), q(0.6), { lit: 0 }));
    const pv = [-0.5, 0, 0.5];
    pn.forEach((p, i) => p.it.to(q(0.6), q(1.8), { x: X(-3.2), z: Z(pv[i]), y: 0.2, s: 0.6 }));
    sibs.forEach((s, i) => s.it.to(q(0.6), q(1.8), { x: X(-2.6), z: Z(pv[i]), y: yT(0.3), s: 0.3 }));
    claimT.it.to(q(0.6), q(1.8), { x: X(-3.2), z: Z(1.0), y: yT(0.5), s: 0.5 });
  }

  // ---- b8 content
  const modePlate = prim(grp(bx(m.ti, 2.4, 0.1, 0.5), bx(m.gold, 2.0, 0.02, 0.1, 0, 0.06, 0)));
  A.modePlate = modePlate.o;
  const hdrSpec = [['outcome', 'hdr:tag'], ['outcome', 'hdr:payload'], ['input', 'input:main'], ['state', 'deps:chain'], ['state', 'hdr:body']];
  const hdr = hdrSpec.map(([f, c]) => tok(f, c));
  const ch2 = tok('state', 'claim:x');
  const ch3 = tok('state', 'claim:y');
  const chainT = tok('state', 'deps:chain');
  const dCopy = tok('state', 'deps:chain');
  A.hdr3 = hdr[3].o; A.chainT = chainT.o;
  {
    const q = (s) => B(8, s);
    modePlate.it.show(q(0.5), { x: X(0), y: 0.2, z: Z(-2.3), s: 1 }, 0.4);
    hdr.forEach((h, i) => h.it.show(q(0.5), { x: X(-2 + i), y: yT(0.6), z: Z(-1.2), s: 0.6, rx: FLAT }, 0.4));
    ch2.it.show(q(0.5), { x: X(-1.8), y: yT(0.5), z: Z(1.0), s: 0.5, rx: FLAT }, 0.4);
    ch3.it.show(q(0.5), { x: X(-0.4), y: yT(0.5), z: Z(1.0), s: 0.5, rx: FLAT }, 0.4);
    chainT.it.show(q(0.5), { x: X(1.0), y: yT(0.8), z: Z(1.0), s: 0.8, rx: FLAT }, 0.4);
    conn(E(claimT), E(ch2), { gold: true }).grow(q(0.8), q(1.4));
    conn(E(ch2), E(ch3), { gold: true }).grow(q(0.8), q(1.4));
    conn(E(ch3), E(chainT), { gold: true }).grow(q(0.8), q(1.4));
    dCopy.it.show(q(1.8), { x: X(1), y: yT(0.6), z: Z(-1.2), s: 0.6, rx: FLAT }, 0.2);
    dCopy.it.to(q(2.0), q(3.0), { z: Z(1.0), s: 0.8, y: yT(0.8) });
    dCopy.it.hide(q(3.0), 0.2);
    finishEq(rgs[1], q(3.0), { x: X(1.0), y: yT(0.8), z: Z(1.0), s: 0.8 });
  }

  // ---- b9 input value link
  const outP = tok('outcome', 'outcome:main');
  const termC = cell(PALETTE.resolve);
  const valL = tok('input', 'val:link');
  const inC = tok('input', 'input:consumer');
  const domC = cell(PALETTE.gold);
  const valR = tok('input', 'val:link');
  const colls = [0, 1, 2].map(() => prim(P.collar()));
  A.outP = outP.o; A.inC = inC.o; A.valR = valR.o; A.coll0 = colls[0].o;
  {
    const q = (s) => B(9, s);
    clear(q(0), [claimT.it, modePlate.it]);
    claimT.it.to(q(0), q(1.2), { x: X(-3), z: Z(-1.4), y: yT(0.8), s: 0.8 });
    modePlate.it.to(q(0), q(1.2), { x: X(3), z: Z(-2.3) });
    outP.it.show(q(0.5), { x: X(-3), y: yT(0.8), z: Z(-0.2), s: 0.8, rx: FLAT }, 0.4);
    termC.show(q(0.5), { x: X(-3.9), y: CELL_Y, z: Z(1.0), s: 0.4 }, 0.4);
    valL.it.show(q(0.5), { x: X(-3), y: yT(0.8), z: Z(1.0), s: 0.8, rx: FLAT }, 0.4);
    inC.it.show(q(0.5), { x: X(3), y: yT(0.8), z: Z(-1.4), s: 0.8, rx: FLAT }, 0.4);
    domC.show(q(0.5), { x: X(3.9), y: CELL_Y, z: Z(1.0), s: 0.4 }, 0.4);
    valR.it.show(q(0.5), { x: X(3), y: yT(0.8), z: Z(1.0), s: 0.8, rx: FLAT }, 0.4);
    colls.forEach((c, i) => c.it.show(q(0.5), { x: X(4.5 + 0.5 * i), y: 0.33, z: Z(1.0), s: 1 }, 0.4));
    conn(E(claimT), E(outP)).grow(q(1.0), q(1.5));
    conn(E(outP), PT(X(-3.9), 0.3, Z(1.0))).grow(q(1.0), q(1.5));
    conn(E(outP), E(valL)).grow(q(1.0), q(1.5));
    conn(E(inC), E(valR)).grow(q(1.0), q(1.5));
    conn(E(inC), PT(X(3.9), 0.3, Z(1.0))).grow(q(1.0), q(1.5));
    conn(E(valR), E(colls[0])).grow(q(1.0), q(1.5));
    valL.it.to(q(2.0), q(3.0), { x: X(0) });
    valR.it.to(q(2.0), q(3.0), { x: X(0) });
    valL.it.hide(q(3.0), 0.2);
    valR.it.hide(q(3.0), 0.2);
    finishEq(rgs[2], q(3.0), { x: X(0), y: yT(0.8), z: Z(1.0), s: 0.8 });
  }

  // ---- b10 retained evidence: consumer proof slab
  const cons = prim(P.kitSlab('decider'));
  const claimC = tok('state', 'claim:consumer', false);
  const covT = tok('state', 'deps:chain', false);
  cons.o.add(claimC.o, covT.o);
  claimC.o.scale.setScalar(0.8); claimC.o.rotation.x = FLAT; claimC.o.position.set(-0.7, 0.62, -0.2);
  covT.o.scale.setScalar(0.6); covT.o.rotation.x = FLAT; covT.o.position.set(0.8, 0.6, -0.2);
  A.cons = cons.o; A.claimC = claimC.o; A.covT = covT.o;
  {
    const q = (s) => B(10, s);
    clear(q(0), [rgs[2].it]);
    cons.it.show(q(0.5), { x: X(-6), y: 0.15, z: Z(0), s: 1, v: 0, e: 0, rg: 0, fo: 0 }, 0.4);
    cons.it.to(q(2.0), q(4.0), { x: X(6) });
    conn(E(cons, [0, 0.6, 0]), E(rgs[2], [0, 0, 0]), { name: 'link1' }).grow(q(0.9), q(1.6));
  }

  // ---- b11 foreign key authentication
  const fs = prim(P.plainSlab({ pins: 4, edge: 'ti' }));
  const regT = tok('state', 'reg:root');
  const fcard = prim(P.card(m.gold, [-0.4, 0.4]));
  const acard = prim(P.card(m.gold, [-0.1, 0.5]));
  A.fs = fs.o; A.fcard = fcard.o; A.acard = acard.o; A.regT = regT.o;
  {
    const q = (s) => B(11, s);
    clear(q(0));
    fs.it.show(q(0.5), { x: X(-4), y: 0.15, z: Z(1.4), s: 1 }, 0.4).to(q(2.0), q(3.2), { x: X(2) });
    fcard.it.show(q(0.5), { x: X(4), y: 0.2, z: Z(-1.4), s: 1 }, 0.4);
    acard.it.show(q(0.5), { x: X(6), y: 0.2, z: Z(-1.4), s: 1 }, 0.4);
    regT.it.show(q(0.5), { x: X(8.5), y: yT(1.2), z: Z(-1.4), s: 1.2, rx: FLAT }, 0.4);
    const fp = [[4, -1.9], [4, -2.3], [8.5, -2.3], [8.5, -2.0]];
    const ap = [[6, -0.9], [6, -0.5], [8.5, -0.5], [8.5, -0.8]];
    [fp, ap].forEach((pts, k) => {
      for (let i = 0; i < 3; i++) conn(PT(X(pts[i][0]), 0.2, Z(pts[i][1])), PT(X(pts[i + 1][0]), 0.2, Z(pts[i + 1][1])), { name: k === 0 && i === 1 ? 'fpath' : undefined }).grow(q(3.0), q(3.5));
    });
    [[5.3, -2.3], [7.2, -2.3], [6.9, -0.5], [7.7, -0.5]].forEach(([u, v]) => cell(PALETTE.gold).show(q(3.0), { x: X(u), y: CELL_Y, z: Z(v), s: 0.3 }, 0.4));
    sweep(q(3.6), q(4.6), X(3.5), X(9.5), Z(-1.4), 2.8, 0.45);
  }

  // ---- b12, b13 registry rows
  const rowCells = (t0, urow, vrow, col, n, tokIdx, tokSpec) => {
    const out = [];
    for (let i = 0; i < n; i++) {
      if (i === tokIdx) {
        const tk = tok('state', tokSpec);
        tk.it.show(t0, { x: X(urow[i]), y: yT(0.6), z: Z(vrow), s: 0.6, rx: FLAT }, 0.4);
        out.push(tk);
      } else {
        const c = cell(col);
        c.show(t0, { x: X(urow[i]), y: CELL_Y, z: Z(vrow), s: 0.4 }, 0.4);
        out.push({ it: c, pos: [X(urow[i]), CELL_Y, Z(vrow)] });
      }
    }
    return out;
  };
  const pairSweep = (q, uArr, u0, u1, v, len) => {
    sweep(q(2.0), q(3.0), X(u0), X(u1), Z(v), len, 0.5);
    uArr.forEach((u, i) => {
      const t = q(2.0) + ((u - u0) / (u1 - u0)) * 1.0;
      pairs[i].it.show(t, { x: X(u), y: 0.2, z: Z(0.8), s: 1, rx: FLAT, lit: 0.5 }, 0.15);
      pairs[i].it.to(t + 0.3, t + 0.45, { lit: 0 });
    });
  };
  A.fields = [];
  {
    const q = (s) => B(12, s);
    clear(q(0), [fs.it, fcard.it, regT.it]);
    const urow = [3.8, 4.8, 5.8, 6.8];
    const pl = prim(grp(bx(m.ti, 4.6, 0.04, 0.8)));
    pl.it.show(q(0.5), { x: X(5.3), y: 0.17, z: Z(0.4), s: 1 }, 0.4);
    const r1 = rowCells(q(0.5), urow, 0.4, PALETTE.ceramic, 4);
    rowCells(q(0.5), urow, 1.2, PALETTE.ceramic, 4);
    A.fieldPos12 = urow.map((u) => [X(u), 0.4, Z(0.4)]);
    A.plate12 = pl.o;
    conn(PT(X(7.2), 0.2, Z(0.4)), PT(X(8.5), 0.2, Z(0.4))).grow(q(1.0), q(1.5));
    conn(PT(X(8.5), 0.2, Z(0.4)), PT(X(8.5), 0.2, Z(-0.8))).grow(q(1.0), q(1.5));
    pairSweep(q, urow, 3.4, 7.2, 0.8, 1.6);
    void r1;
    // b13
    const w = (s) => B(13, s);
    clear(w(0), [regT.it]);
    const u6 = [2.5, 3.5, 4.5, 5.5, 6.5, 7.5];
    const pl2 = prim(grp(bx(m.ti, 6.4, 0.04, 0.8)));
    pl2.it.show(w(0.5), { x: X(5.0), y: 0.17, z: Z(0.4), s: 1 }, 0.4);
    rowCells(w(0.5), u6, 0.4, PALETTE.ceramic, 6, 3, 'adapter:digest');
    rowCells(w(0.5), u6, 1.2, PALETTE.ceramic, 6, 3, 'adapter:digest');
    A.plate13 = pl2.o;
    conn(PT(X(7.8), 0.2, Z(0.4)), PT(X(8.5), 0.2, Z(0.4))).grow(w(1.0), w(1.5));
    conn(PT(X(8.5), 0.2, Z(0.4)), PT(X(8.5), 0.2, Z(-0.8))).grow(w(1.0), w(1.5));
    pairSweep(w, u6, 2.2, 7.8, 0.8, 1.6);
  }

  // ---- b14 foreign proof parsed
  const ptU = [-2.0, -1.5, -1.0, -0.5, 0.0];
  const scU = [0.5, 1.0, 1.5, 2.0];
  const pts = [];
  const scal = [];
  const edges = [];
  const stop = prim(grp(bx(m.ti, 0.15, 0.5, 0.8)));
  const parseTray = prim(grp(bx(m.ti, 5.2, 0.04, 1.1)));
  const fmtCard = prim(P.card(m.ti, [-0.3, 0.3]));
  const rowItems = [];
  A.parseTray = parseTray.o; A.fmt = fmtCard.o; A.stop = stop.o;
  {
    const q = (s) => B(14, s);
    clear(q(0));
    fs.it.show(q(0.5), { x: X(-4), y: 0.15, z: Z(0), s: 1 }, 0.4);
    parseTray.it.show(q(0.5), { x: X(0.15), y: 0.17, z: Z(0), s: 1 }, 0.4);
    ptU.forEach((u, i) => {
      const t = tok('state', 'pt:' + i);
      t.it.show(q(0.5), { x: X(u), y: yT(0.4), z: Z(0), s: 0.4, rx: FLAT }, 0.4);
      pts.push(t); rowItems.push(t.it);
    });
    scU.forEach((u) => {
      const c = cell(PALETTE.ceramic);
      c.show(q(0.5), { x: X(u), y: CELL_Y, z: Z(0), s: 0.4 }, 0.4);
      scal.push(c); rowItems.push(c);
    });
    stop.it.show(q(0.5), { x: X(2.4), y: 0.4, z: Z(0), s: 1 }, 0.4);
    rowItems.push(stop.it, parseTray.it);
    fmtCard.it.show(q(0.5), { x: X(-1), y: 0.2, z: Z(-1.6), s: 1 }, 0.4);
    const all = [...ptU, ...scU];
    all.forEach((u, i) => {
      const e = plate();
      const t = q(2.0) + ((u + 2.3) / 4.8) * 1.6;
      e.show(t, { x: X(u), y: 0.19, z: Z(0), s: 0.5 }, 0.15);
      edges.push(e); rowItems.push(e);
    });
    sweep(q(2.0), q(3.6), X(-2.3), X(2.5), Z(0), 1.2, 0.5);
  }

  // ---- b15 transcript
  const inCells = [];
  const chCells = [];
  const trCard = prim(P.card(m.ti, [-0.3, 0.3]));
  A.trCard = trCard.o;
  {
    const q = (s) => B(15, s);
    clear(q(0), [fs.it, ...rowItems]);
    [3.0, 3.5, 4.0].forEach((u) => {
      const c = cell(PALETTE.steel);
      c.show(q(2.0), { x: X(u), y: CELL_Y, z: Z(0), s: 0.4 }, 0.4);
      inCells.push(c); rowItems.push(c);
    });
    trCard.it.show(q(0.5), { x: X(0.5), y: 0.2, z: Z(-1.6), s: 1 }, 0.4);
    conn(PT(X(0.5), 0.2, Z(-1.1)), PT(X(0.5), 0.2, Z(-0.2))).grow(q(1.0), q(1.5));
    sweep(q(3.4), q(5.0), X(-2.3), X(4.2), Z(0), 1.2, 0.5);
    [5.0, 5.5, 6.0].forEach((u, i) => {
      const c = cell(PALETTE.gold);
      c.show(q(4.2 + 0.3 * i), { x: X(u), y: CELL_Y, z: Z(0), s: 0.4 }, 0.4);
      chCells.push(c); rowItems.push(c);
      conn(PT(X(u - (i ? 0.5 : 1.0)), 0.2, Z(0)), PT(X(u), 0.2, Z(0))).grow(q(4.2 + 0.3 * i), q(4.6 + 0.3 * i));
    });
  }

  // ---- b16 arithmetic
  const evals = [];
  const sumT = [];
  const resA = tok('state', 'opening:eq');
  const resB = tok('state', 'opening:eq');
  const arithTray = prim(grp(bx(m.slate, 9.0, 0.04, 2.2)));
  A.arith = arithTray.o; A.resA = resA.o; A.resB = resB.o;
  {
    const q = (s) => B(16, s);
    clear(q(0), rowItems);
    const cx = X(2.0);
    for (const it of rowItems) {
      const c = it.cur;
      it.to(q(0.2), q(1.2), { x: cx + (c.x - cx) * 0.6, z: Z(-1.6), y: 0.16 + (c.y - 0.16) * 0.6, s: c.s * 0.6 });
    }
    arithTray.it.show(q(1.2), { x: X(0.2), y: 0.17, z: Z(0.6), s: 1 }, 0.4);
    [-3.75, -3.25, -2.75, -2.25].forEach((u) => { const c = cell(PALETTE.ceramic); c.show(q(1.4), { x: X(u), y: CELL_Y, z: Z(0.6), s: 0.4 }, 0.4); evals.push(c); });
    [-0.5, 0, 0.5].forEach((u, i) => { const t = tok('state', 'pt2:' + i); t.it.show(q(1.4), { x: X(u), y: yT(0.4), z: Z(0.6), s: 0.4, rx: FLAT }, 0.4); sumT.push(t); });
    conn(PT(X(-0.8), 0.2, Z(1.05)), PT(X(0.8), 0.2, Z(1.05))).grow(q(1.4), q(1.8));
    resA.it.show(q(1.4), { x: X(3.5), y: yT(0.5), z: Z(0.2), s: 0.5, rx: FLAT }, 0.4);
    resB.it.show(q(1.4), { x: X(3.5), y: yT(0.5), z: Z(1.0), s: 0.5, rx: FLAT }, 0.4);
    [5.0, 5.5, 6.0].forEach((u) => {
      const x0 = cx + (u - 2.0) * 0.6;
      conn(PT(x0, 0.2, Z(-1.6)), PT(X(-3), 0.2, Z(0.3))).grow(q(1.4), q(2.0));
      conn(PT(x0, 0.2, Z(-1.6)), PT(X(0), 0.2, Z(0.3))).grow(q(1.4), q(2.0));
    });
    sweep(q(2.0), q(3.0), X(-4.2), X(4.2), Z(0.6), 2.2, 0.5);
    resB.it.to(q(3.0), q(4.0), { z: Z(0.2) });
    resB.it.hide(q(4.0), 0.2);
    finishEq(rgs[3], q(4.0), { x: X(3.5), y: yT(0.5), z: Z(0.2), s: 0.5 });
    A.chal16 = [5.0, 5.5, 6.0].map((u) => [cx + (u - 2.0) * 0.6, 0.35, Z(-1.6)]);
    A.eval16 = [X(-3), 0.4, Z(0.6)];
    A.sum16 = [X(0), 0.4, Z(0.6)];
  }

  // ---- b17 continued verification
  const nodeB = prim(P.plainSlab({ pins: 5 }));
  const st1 = tok('state', 'fv:state1');
  const st2 = tok('state', 'fv:state1');
  const commit = tok('state', 'fv:commit');
  const hexSock = prim(grp((() => { const h = new THREE.Mesh(g.hex, m.ink); h.scale.set(1, 0.02, 1); return h; })()));
  A.nodeB = nodeB.o; A.st1 = st1.o; A.st2 = st2.o; A.commit = commit.o;
  const fvCells = [];
  {
    const q = (s) => B(17, s);
    clear(q(0));
    nodeA.it.show(q(0.5), { x: X(-3.5), y: 0.15, z: Z(-0.8), s: 0.6, v: 0, e: 0, rg: 0, fo: 0 }, 0.4);
    nodeB.it.show(q(0.5), { x: X(0.5), y: 0.15, z: Z(-0.8), s: 0.6 }, 0.4);
    st1.it.show(q(0.5), { x: X(-2.6), y: 0.55, z: Z(-0.8), s: 0.5, rx: FLAT }, 0.4);
    st2.it.show(q(0.5), { x: X(-0.4), y: 0.55, z: Z(-0.8), s: 0.5, rx: FLAT }, 0.4);
    [[-3.0, PALETTE.gold], [-1.8, PALETTE.steel], [-0.6, PALETTE.ceramic]].forEach(([u, col]) => {
      const c = cell(col); c.show(q(0.5), { x: X(u), y: CELL_Y, z: Z(1.4), s: 0.4 }, 0.4); fvCells.push(c);
    });
    commit.it.show(q(0.5), { x: X(0.6), y: yT(0.5), z: Z(1.4), s: 0.5, rx: FLAT }, 0.4);
    hexSock.it.show(q(0.5), { x: X(5), y: 0.17, z: Z(-0.8), s: 1 }, 0.4);
    st2.it.to(q(2.0), q(3.0), { x: X(-2.6) });
    st2.it.hide(q(3.0), 0.2);
    finishEq(rgs[4], q(3.0), { x: X(-2.6), y: 0.55, z: Z(-0.8), s: 0.5 });
  }

  // ---- b18 final foreign check
  const oblig = cell(PALETTE.gold);
  const card1 = prim(P.card(m.ti, [-0.3, 0.3]));
  const card2 = prim(P.card(m.ti, [-0.3, 0.3]));
  const blockMat = P.ownGold();
  const block = prim(P.dimFx(grp(bx(blockMat, 1, 0.5, 1, 0, 0.25, 0), (() => { const h = new THREE.Mesh(g.hex, blockMat); h.scale.set(0.4, 0.25, 0.4); h.position.y = 0.6; return h; })()), blockMat, PALETTE.paperWarm));
  const vtray = prim(grp(bx(m.slate, 8.4, 0.04, 4.6)));
  A.card1 = card1.o; A.block = block.o; A.vtray = vtray.o;
  {
    const q = (s) => B(18, s);
    clear(q(0));
    vtray.it.show(q(0.3), { x: X(-0.6), y: 0.17, z: Z(-0.1), s: 1 }, 0.4);
    oblig.show(q(0.5), { x: X(-4), y: CELL_Y, z: Z(0), s: 0.4 }, 0.4);
    card1.it.show(q(0.5), { x: X(-4), y: 0.22, z: Z(-1.6), s: 1 }, 0.4);
    card2.it.show(q(0.5), { x: X(-1.5), y: 0.22, z: Z(-1.6), s: 1 }, 0.4);
    nodeA.it.show(q(0.5), { x: X(2), y: 0.15, z: Z(-0.8), s: 0.5, v: 0, e: 0, rg: 1, fo: 0 }, 0.4);
    A.ringSlab = nodeA.o;
    block.it.show(q(0.5), { x: X(2), y: 0.15, z: Z(1.2), s: 1, dim: 0 }, 0.4);
    const c1 = conn({ o: { position: new THREE.Vector3(X(-4), CELL_Y, Z(0)) }, p: [0.2, 0, 0] }, PT(X(1.25), CELL_Y, Z(-0.8)));
    const c2 = conn({ o: { position: new THREE.Vector3(X(-4), CELL_Y, Z(0)) }, p: [0.2, 0, 0] }, PT(X(1.5), CELL_Y, Z(1.2)));
    c1.grow(q(1.0), q(1.6)); c2.grow(q(1.0), q(1.6));
    card2.it.to(q(2.0), q(3.0), { x: X(-4) });
    card2.it.hide(q(3.0), 0.2);
    cardRing.it.show(q(3.0), { x: X(-4), y: 0.25, z: Z(-1.6), s: 1, rx: FLAT, lit: 0.5 }, 0.2);
    cardRing.it.to(q(3.3), q(3.45), { lit: 0 });
    block.it.to(q(2.0), q(2.6), { dim: 1 });
    c1.to(q(3.6), q(3.65), { lit: 1 });
    sweep(q(3.6), q(4.6), X(-3.6), X(1.2), Z(-0.4), 1.6, 0.5);
    nodeA.it.to(q(4.6), q(4.8), { fo: 1 });
    A.oblig = [X(-4), CELL_Y, Z(0)];
  }

  // ---- b19 admit the foreign child
  const housing = prim(grp(bx(m.slate, 3.9, 0.12, 1.5, 0, 0.06, 0), bx(m.ti, 3.9, 0.06, 0.08, 0, 0.13, 0.71), bx(m.ti, 3.9, 0.06, 0.08, 0, 0.13, -0.71), bx(m.ti, 0.08, 0.06, 1.5, -1.91, 0.13, 0), bx(m.ti, 0.08, 0.06, 1.5, 1.91, 0.13, 0), bx(m.ink, 1.55, 0.02, 1.05, -0.95, 0.125, 0.1), bx(m.ink, 1.55, 0.02, 1.05, 0.95, 0.125, 0.1), bx(m.gold, 0.9, 0.3, 0.45, 0, 0.27, -0.45)));
  const fs2 = prim(P.plainSlab({ pins: 4, edge: 'ti' }));
  const adCard = P.card(m.ti, [-0.3, 0.3]);
  adCard.scale.setScalar(0.5); adCard.position.set(-0.75, 0.43, -0.4);
  const inCubes = [-0.8, -0.4, 0].map((x) => bx(m.input, 0.25, 0.25, 0.25, x, 0.55, 0.3));
  const claimF = tok('state', 'claim:foreign', false);
  claimF.o.scale.setScalar(0.8); claimF.o.rotation.x = FLAT; claimF.o.position.set(0.8, 0.5, 0);
  const frameF = P.frameTi(1.1, 1.1);
  frameF.position.set(0.8, 0.4, 0);
  fs2.o.add(adCard, ...inCubes, claimF.o, frameF);
  A.housing = housing.o; A.adCard = adCard; A.inCube = inCubes[1]; A.claimF = claimF.o; A.fs2 = fs2.o;
  {
    const q = (s) => B(19, s);
    clear(q(0));
    housing.it.show(q(0.5), { x: X(6), y: 0.15, z: Z(0), s: 1 }, 0.4);
    nodeA.it.show(q(0.5), { x: X(6) - 0.95, y: 0.29, z: Z(0.1), s: 0.5, v: 0, e: 0, rg: 0, fo: 0 }, 0.4);
    fs2.it.show(q(0.5), { x: X(1), y: 0.15, z: Z(0), s: 1 }, 0.4);
    fs2.it.to(q(2.0), q(3.6), { x: X(6) + 0.95, y: 0.29, z: Z(0.1), s: 0.5 });
    conn(PT(11.5, 0.45, 3.05), PT(11.1, 0.4, 2.6), { name: 'adConn' }).grow(q(3.6), q(4.4));
    conn(PT(12.4, 0.45, 3.05), PT(11.35, 0.4, 2.6), { name: 'foConn' }).grow(q(3.6), q(4.4));
    A.sock2 = new THREE.Object3D();
    A.sock2.position.set(X(6) + 0.95, 0.3, Z(0.1) - 0.9);
    sc.add(A.sock2);
  }

  // ---------------------------------------------------------------- b20 record handoff
  const rec = prim(makeDrawer(P, { struck: false }), false);
  const recClaim = P.tok('state', 'claim:main');
  recClaim.scale.setScalar(0.8); recClaim.position.set(0, 0, 0.11);
  const recOut = P.tok('outcome', 'outcome:main');
  recOut.scale.setScalar(0.6); recOut.rotation.x = FLAT; recOut.position.set(SEAT_OUT[0], SEAT_OUT[1], SEAT_OUT[2]);
  const recCollar = P.collar();
  recCollar.position.set(SEAT_COLLAR[0], SEAT_COLLAR[1], SEAT_COLLAR[2]);
  rec.o.add(recClaim, recOut, recCollar);
  A.rec = rec.o; A.recClaim = recClaim; A.recOut = recOut; A.recCollar = recCollar;
  {
    const q = (s) => B(20, s);
    clear(q(0));
    const closeBy = DRW_CLOSED[2] - DRW_OUT[2];
    drw.it.to(q(2.0), q(3.4), { z: DRW_CLOSED[2] });
    for (const it of [claim.it, cO.it, collarCp.it]) it.to(q(2.0), q(3.4), { z: it.cur.z + closeBy });
    rec.it.show(q(2.0), { x: DRW_OUT[0], y: DRW_OUT[1] + 0.04, z: DRW_OUT[2] + 0.06, s: 1 }, 0.3);
    rec.it.to(q(2.3), q(3.4), { x: 3, y: 0.7, z: 1.0 });
    rec.it.to(q(10.0), q(12.2), { x: ctx.plates.out.x, y: 0.7, z: 1.0 });
  }

  return { A, vk };
}
