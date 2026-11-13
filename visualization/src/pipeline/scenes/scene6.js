// Scene 6: the transaction. The decider seal and the four public values are packed into an envelope,
// the call transcript binds them, the fee is paid in DUST, and the closed envelope crosses to the chain.
// Every motion is a pure function of the beat index and the time inside the beat (STORYBOARD scene 6).
import * as THREE from 'three/webgpu';
import { PALETTE, materials, makeDigestToken, phase, ramp, lerp, clamp01 } from '../theme.js';
import { makeProofSlab, makeVerifierKeyCard } from '../kit.js';
import { ident } from '../labels.js';
import { NARRATION } from './scene6/text.js';

const PI = Math.PI;
const ENV_X = -9;                       // envelope centre on the prover side
const ENV2_Z = 4.0;                     // comparison envelope
const SOCK_X = [-1.9, -0.9, 0.1, 1.2];  // value sockets, env-local x
const SOCK_Z = 1.3;
const SEAL_AT = [-0.7, 0.1, -0.6];      // seal in its pocket
const CALL_AT = [2.0, 0.4];             // call-data position (x, z)
const FEE_AT = [2.0, -0.9];             // fee recess (x, z)
const TOKEN_Y = 0.432;                  // token centre on a socket
const CALL_Y = 0.402;                   // token centre on the call-data strip
const LID_Y = 0.7;                     // lid centre when closed
const DOOR = [13, 1.1, 0];
const RACK = [-9, 4.2, -5.4];          // proving key rack, off-chain side
const VK = [13, 3.0, -2.6];             // decider verifier key card beside the door
const ROW_Z = -3.2;                     // row of objects that stay off-chain (beat 6.4)
const LINT_Y = 2.2;                     // underside of the door lintel
const CONTENT = { prog_d: 'prog:main', input_d: 'input:main', outcome_d: 'outcome:main', n_steps: 'n_steps:main' };

const D = [32, 23, 27, 30];
const T = {
  arrive: { sealA: 2, sealB: 4, tok0: 4.4, tokStep: 0.3, tokDur: 1.1, lockA: 11, lockB: 11.6, sweepB: 12, tabB: 12.3 },
};

let ctx = null;
let root = null;
let res = null;
let E = null;    // the honest envelope
let E2 = null;   // the comparison envelope
let rack = null;
let door = null;
let dust = null;
let lid = null;
let lineData = null;
let lineVk = null;
let vkCard = null;
let row = null;
let stopBar = null;
let lineTick = null;
let anchors = {};

const dummy = new THREE.Object3D();
const VK_STATE = { lit: 0 };          // reused so seek allocates nothing
const q1 = new THREE.Quaternion();
const vA = new THREE.Vector3();
const vB = new THREE.Vector3();
const vC = new THREE.Vector3();
const ZAXIS = new THREE.Vector3(0, 0, 1);

const smooth = (x, a, b) => {
  const k = clamp01((x - a) / (b - a));
  return k * k * (3 - 2 * k);
};

function mk(geo, mat, w, h, d, x, y, z, parent) {
  const m = new THREE.Mesh(geo, mat);
  m.scale.set(w, h, d);
  m.position.set(x, y, z);
  if (parent) parent.add(m);
  return m;
}

// Static instanced boxes from rects [x0, x1, y0, y1, z0, z1].
function inst(geo, mat, rects, parent) {
  const m = new THREE.InstancedMesh(geo, mat, rects.length);
  rects.forEach((r, i) => {
    dummy.position.set((r[0] + r[1]) / 2, (r[2] + r[3]) / 2, (r[4] + r[5]) / 2);
    dummy.rotation.set(0, 0, 0);
    dummy.scale.set(r[1] - r[0], r[3] - r[2], r[5] - r[4]);
    dummy.updateMatrix();
    m.setMatrixAt(i, dummy.matrix);
  });
  m.instanceMatrix.needsUpdate = true;
  m.frustumCulled = false;
  if (parent) parent.add(m);
  return m;
}

// A thin box from a to a + (b - a) * k.
function segment(a, b, k, mesh) {
  vA.set(a[0], a[1], a[2]);
  vB.set(b[0], b[1], b[2]);
  vC.copy(vB).sub(vA);
  const len = vC.length() * k;
  mesh.visible = len > 0.02;
  if (!mesh.visible) return;
  vC.normalize();
  q1.setFromUnitVectors(ZAXIS, vC);
  mesh.quaternion.copy(q1);
  mesh.position.copy(vA).addScaledVector(vC, len / 2);
  mesh.scale.set(0.07, 0.07, len);
}

function setTeeth(mesh, t1x, t1z) {
  const xs = [t1x, SOCK_X[1], SOCK_X[2], SOCK_X[3], 0.6];
  const zs = [t1z, 0.15, 0.15, 0.15, -0.15];
  for (let i = 0; i < 5; i++) {
    dummy.position.set(xs[i], 0, zs[i]);
    dummy.rotation.set(0, 0, 0);
    dummy.scale.setScalar(0.2);
    dummy.updateMatrix();
    mesh.setMatrixAt(i, dummy.matrix);
  }
  mesh.instanceMatrix.needsUpdate = true;
}

// One envelope: open case with deck, rim, sockets, locking bar, address tab and the seated contents.
function makeEnvelope(withSweep, sealSrc, otherToken) {
  const g = new THREE.Group();
  const o = {};
  mk(res.UB, res.slate, 5.2, 0.1, 3.6, 0, 0.05, 0, g);
  inst(res.UB, res.slate, [
    [0.9, 2.6, 0.1, 0.3, -1.8, 1.8], [-2.6, -2.3, 0.1, 0.3, -1.8, 1.8],
    [-2.3, 0.9, 0.1, 0.3, -1.8, -1.7], [-2.3, 0.9, 0.1, 0.3, 0.5, 1.8],
  ], g);
  inst(res.UB, res.ti, [
    [-2.6, 2.6, 0.3, 0.66, -1.8, -1.72], [-2.6, 2.6, 0.3, 0.66, 1.72, 1.8],
    [-2.6, -2.52, 0.3, 0.66, -1.72, 1.72], [2.52, 2.6, 0.3, 0.66, -1.72, 1.72],
  ], g);
  mk(res.UB, res.ink, 4.3, 0.03, 1.0, -0.35, 0.315, SOCK_Z, g);
  mk(res.UB, res.ink, 1.1, 0.03, 0.95, CALL_AT[0], 0.315, CALL_AT[1], g);
  mk(res.DISC, res.ink, 0.96, 0.03, 0.96, FEE_AT[0], 0.315, FEE_AT[1], g);
  const hex = new THREE.InstancedMesh(res.HEX, res.ti, 3);
  for (let i = 0; i < 3; i++) {
    dummy.position.set(SOCK_X[i], 0.345, SOCK_Z);
    dummy.rotation.set(0, PI / 6, 0);
    dummy.scale.set(0.92, 0.03, 0.92);
    dummy.updateMatrix();
    hex.setMatrixAt(i, dummy.matrix);
  }
  hex.frustumCulled = false;
  g.add(hex);
  mk(res.UB, res.ti, 0.7, 0.03, 0.5, SOCK_X[3], 0.345, SOCK_Z, g);
  o.tab = mk(res.UB, res.tabMat, 2.4, 0.1, 0.5, 0, 0.71, -1.8, g);

  o.lock = new THREE.Group();
  o.lock.position.set(-0.3, 0.36, 0.85);
  mk(res.UB, res.ti, 4.4, 0.12, 0.2, 0, 0, 0, o.lock);
  o.teeth = new THREE.InstancedMesh(res.UB, res.ti, 5);
  o.teeth.frustumCulled = false;
  o.lock.add(o.teeth);
  g.add(o.lock);
  if (withSweep) {
    o.sweep = mk(res.UB, res.sweep, 0.1, 0.16, 0.26, 0, 0, 0, o.lock);
    o.sweep.visible = false;
  }

  o.seal = sealSrc ? sealSrc() : null;
  g.add(o.seal);
  o.seal.position.set(...SEAL_AT);
  o.tokens = [];
  const faces = [['program', 'prog:main'], ['input', 'input:main'], ['outcome', otherToken ? 'outcome:other' : 'outcome:main']];
  faces.forEach(([fam, content], i) => {
    const tk = makeDigestToken({ family: fam, content, size: 0.8 });
    tk.rotation.x = -PI / 2;
    tk.position.set(SOCK_X[i], TOKEN_Y, SOCK_Z);
    g.add(tk);
    o.tokens.push(tk);
  });
  o.collar = mk(res.HEX, res.gold, 0.56, 0.14, 0.56, SOCK_X[3], 0.43, SOCK_Z, g);
  o.collar.rotation.y = PI / 6;
  g.add(o.collar);
  o.group = g;
  root.add(g);
  return o;
}

function makeSeal() {
  const s = makeProofSlab({ variant: 'decider', pinKinds: ['collar', 'collar', 'collar', 'bar'] });
  s.userData.setState({ pins: ['program', 'input', 'outcome', null], ring: { show: 1, folds: 12, closed: 1 } });
  return s;
}

// The comparison seal shares geometry and materials with the first one (same proof).
function cloneSeal(src) {
  const ud = src.userData;
  const ringIdx = src.children.indexOf(ud.ring);
  src.userData = {};
  const c = src.clone(true);
  src.userData = ud;
  const ring = c.children[ringIdx];
  ring.children.forEach((m, i) => { if (i >= 2) m.visible = false; });
  return c;
}

export default {
  id: 'transaction',
  title: 'The transaction',
  beats: [
    {
      id: 'pack-the-call',
      title: 'The call',
      duration: D[0],
      where: 'off',
      narration: NARRATION[0],
      labels: [
        { id: 'seal', text: 'Decider proof', anchor: 'aSeal', kind: 'state', dy: 0.6 },
        { id: 'prog', text: ident('prog_d'), anchor: 'tok0', kind: 'program', dy: 0.8, from: 10.5 },
        { id: 'input', text: ident('input_d'), anchor: 'tok1', kind: 'input', dy: 0.8, from: 10.5 },
        { id: 'outcome', text: ident('outcome_d'), anchor: 'tok2', kind: 'outcome', dy: 0.8, from: 10.5 },
        { id: 'steps', text: ident('n_steps'), anchor: 'collar', kind: 'state', dy: 0.8, from: 10.5 },
        { id: 'addr', text: ident('SherlockRun.acceptRun'), anchor: 'tab', kind: 'neutral', dy: 0.6 },
      ],
    },
    {
      id: 'refuse-a-swapped-value',
      title: 'Swapped value refused',
      duration: D[1],
      where: 'off',
      narration: NARRATION[1],
      labels: [
        { id: 'transcript', text: 'Call transcript', anchor: 'strip1', kind: 'neutral', dy: 0.6 },
        { id: 'same', text: 'Same proof', anchor: 'aSeal2', kind: 'state', dy: 0.6, from: 1 },
        { id: 'changed', text: 'Changed outcome', anchor: 'changed', kind: 'outcome', dy: 0.3, from: 3.4 },
        { id: 'refused', text: 'Refused', anchor: 'refused', kind: 'error', dy: 0.3, from: 3.4 },
      ],
    },
    {
      id: 'keep-the-contract',
      title: 'One contract',
      duration: D[2],
      where: 'off',
      narration: NARRATION[2],
      labels: [
        { id: 'prog', text: ident('prog_d'), anchor: 'tok0', kind: 'program', dy: 0.8, from: 3.2 },
        { id: 'call', text: 'Call data', anchor: 'cstrip', kind: 'neutral', dy: 0.7, from: 3.2 },
        { id: 'keys', text: 'Proving keys stay', anchor: 'rackCard', kind: 'state', dy: 0.5, from: 0.8 },
        { id: 'vk', text: 'Decider verifier key', anchor: 'vk', kind: 'state', dy: 0.4, from: 0.8 },
        { id: 'door', text: ident('SherlockRun'), anchor: 'doorFront', kind: 'neutral', dy: 0.5, from: 0.8 },
      ],
    },
    {
      id: 'cross-to-slate',
      title: 'Crossing to the chain',
      duration: D[3],
      where: 'cross',
      narration: NARRATION[3],
      labels: [
        { id: 'dust', text: 'DUST', anchor: 'dust', kind: 'neutral', dy: 0.9, from: 0.6, to: 6.0 },
        { id: 'chip', text: 'Stays: Sherlock chip', anchor: 'housing', kind: 'neutral', dy: 0.5, from: 0.6, to: 8.0 },
        { id: 'keys', text: 'Stays: proving keys', anchor: 'rackCard', kind: 'state', dy: 0.5, from: 0.6, to: 8.0 },
        { id: 'tape', text: 'Stays: input, witness, trace', anchor: 'tapeRow', kind: 'input', dy: 0.5, from: 0.6, to: 8.0 },
        { id: 'seal', text: 'Crosses: decider proof', anchor: 'aSeal', kind: 'state', dy: 0.5, from: 6.0 },
        { id: 'vals', text: 'Crosses: four public values', anchor: 'sockets', kind: 'neutral', dy: 0.5, from: 6.0 },
      ],
    },
  ],

  build(c) {
    ctx = c;
    root = new THREE.Group();
    ctx.scene.add(root);
    const sealMat = new THREE.MeshBasicMaterial({ color: PALETTE.onSurface });
    res = {
      UB: new THREE.BoxGeometry(1, 1, 1),
      HEX: new THREE.CylinderGeometry(0.5, 0.5, 1, 6),
      DISC: new THREE.CylinderGeometry(0.5, 0.5, 1, 28),
      RING: new THREE.TorusGeometry(0.4, 0.03, 6, 28),
      slate: materials.slate(),
      ti: materials.titanium(),
      ink: materials.ink(),
      ceramic: materials.ceramic(),
      gold: materials.gold(),
      tabMat: materials.gold(),
      sweep: materials.gold(),
      hatch: materials.family('error', { lit: 0.6 }),
      cards: materials.gold(),
      doorRim: materials.titanium(),
      line: sealMat,
      inputMat: materials.family('input'),
    };
    res.RING.rotateX(PI / 2);
    res.sweep.emissive.setHex(PALETTE.goldBright);
    res.sweep.emissiveIntensity = 1;
    res.tabMat.emissive.setHex(PALETTE.goldBright);
    res.cards.emissive.setHex(PALETTE.goldBright);
    res.doorRim.emissive.setHex(PALETTE.goldBright);

    const seal1 = makeSeal();
    E = makeEnvelope(true, () => seal1, false);
    E.group.position.set(ENV_X, 0, 0);
    E2 = makeEnvelope(false, () => cloneSeal(seal1), true);
    E2.group.position.set(ENV_X, 0, ENV2_Z);
    E2.hatch = inst(res.UB, res.hatch, [[-0.05, 0.05, 0.41, 0.43, -1.2, 1.2], [-0.05, 0.05, 0.41, 0.43, -1.2, 1.2], [-0.05, 0.05, 0.41, 0.43, -1.2, 1.2]], E2.seal);
    for (let i = 0; i < 3; i++) {
      dummy.position.set((i - 1) * 0.5, 0.42, 0);
      dummy.rotation.set(0, PI / 4, 0);
      dummy.scale.set(0.1, 0.02, 2.4);
      dummy.updateMatrix();
      E2.hatch.setMatrixAt(i, dummy.matrix);
    }
    E2.hatch.instanceMatrix.needsUpdate = true;
    E2.changed = new THREE.Object3D();
    E2.changed.position.set(SOCK_X[2], 0.3, SOCK_Z + 0.65);
    E2.group.add(E2.changed);
    E2.tick = mk(res.UB, res.line, 0.07, 0.07, 1, 0, 0, 0, E2.group);
    E2.refused = new THREE.Object3D();
    E2.refused.position.set(2.95, 0.3, 0);
    E2.group.add(E2.refused);
    E2.strip = new THREE.Object3D();
    E.strip = new THREE.Object3D();
    E.strip.position.set(2.1, 0.4, 1.55);
    E.group.add(E.strip);

    // Dust, the fee object, travels with the envelope once it sits in the recess.
    dust = new THREE.Group();
    dust.add(mk(res.DISC, res.ceramic, 0.8, 0.12, 0.8, 0, 0, 0, null));
    dust.add(mk(res.RING, res.gold, 1, 1, 1, 0, 0.06, 0, null));
    E.group.add(dust);

    // Lid with cut-outs over the sockets, the call-data strip, the fee recess and the tab.
    lid = new THREE.Group();
    const y = [-0.04, 0.04];
    const R = (x0, x1, z0, z1) => [x0, x1, y[0], y[1], z0, z1];
    inst(res.UB, res.slate, [
      R(-2.6, 1.5, -1.5, 0.85), R(2.5, 2.6, -1.5, 0.85), R(1.5, 2.5, -0.4, -0.1), R(1.5, 2.5, -1.5, -1.4),
      R(-2.6, -1.2, -1.8, -1.5), R(1.2, 2.6, -1.8, -1.5),
      R(-2.6, -2.45, 0.85, 1.8), R(1.75, 2.6, 0.85, 1.8), R(-2.45, 1.75, 1.75, 1.8),
    ], lid);
    E.group.add(lid);

    // Key rack and contract entrance.
    rack = new THREE.Group();
    rack.position.set(...RACK);
    mk(res.UB, res.ti, 7, 0.15, 0.15, 0, 0, 0, rack);
    inst(res.UB, res.ti, [[-3.575, -3.425, -4.2, 0, -0.075, 0.075], [3.425, 3.575, -4.2, 0, -0.075, 0.075]], rack);
    [-2.2, 0, 2.2].forEach((x) => mk(res.UB, res.cards, 1.6, 1.0, 0.08, x, -0.65, 0, rack));
    const notch = [[-2.2, -0.4], [-2.2, 0], [-2.2, 0.4], [0, -0.2], [0, 0.2], [2.2, 0]];
    inst(res.UB, res.ink, notch.map(([cx, dx]) => [cx + dx - 0.11, cx + dx + 0.11, -1.09, -0.95, 0.03, 0.07]), rack);
    root.add(rack);
    door = new THREE.Group();
    door.position.set(...DOOR);
    inst(res.UB, res.slate, [
      [-0.15, 0.15, -1.1, 1.1, -2.4, -2.1], [-0.15, 0.15, -1.1, 1.1, 2.1, 2.4], [-0.15, 0.15, 1.1, 1.4, -2.4, 2.4],
    ], door);
    inst(res.UB, res.doorRim, [
      [-0.18, 0.18, 1.4, 1.46, -2.45, 2.45], [-0.2, 0.2, -1.1, -1.04, -2.45, -2.05], [-0.2, 0.2, -1.1, -1.04, 2.05, 2.45],
    ], door);
    root.add(door);
    // Decider verifier key card on a post beside the door (the one key on-chain).
    vkCard = makeVerifierKeyCard({ size: 1 });
    vkCard.position.set(...VK);
    mk(res.UB, res.ti, 0.12, VK[1] - 0.42, 0.12, 0, -(VK[1] - 0.42) / 2 - 0.42, 0, vkCard);
    root.add(vkCard);

    // Beat 6.4: the row of objects that stay off-chain.
    row = new THREE.Group();
    const hz = ROW_Z;
    // Leaf housing with its lid and the board outline, scene 3 drawn at scale 0.12.
    mk(res.UB, res.ceramic, 3.24, 0.3, 0.96, -12, 0.15, hz, row);
    mk(res.UB, res.ti, 3.3, 0.06, 1.0, -12, 0.33, hz, row);
    inst(res.UB, res.ink, [
      [-13.92, -10.08, 0, 0.03, hz - 0.72, hz - 0.68], [-13.92, -10.08, 0, 0.03, hz + 0.68, hz + 0.72],
      [-13.92, -13.88, 0, 0.03, hz - 0.68, hz + 0.68], [-10.12, -10.08, 0, 0.03, hz - 0.68, hz + 0.68],
    ], row);
    // Input cube.
    mk(res.UB, res.inputMat, 0.6, 0.6, 0.6, -8.5, 0.3, hz, row);
    // Tape strip of four frames at scale 0.5 with a break mark.
    const fr = [];
    for (let i = 0; i < 4; i++) fr.push([-5.5 + (i - 1.5) * 0.5 - 0.225, -5.5 + (i - 1.5) * 0.5 + 0.225, 0.05, 0.08, hz - 0.3, hz + 0.3]);
    inst(res.UB, res.ceramic, fr, row);
    inst(res.UB, res.ink, [[-4.56, -4.5, 0.1, 0.13, hz - 0.35, hz + 0.35], [-4.4, -4.34, 0.1, 0.13, hz - 0.35, hz + 0.35]], row);
    root.add(row);
    stopBar = mk(res.UB, res.ti, 0.2, 0.5, 4, -3.2, 0.25, hz, root);

    // Connectors.
    lineData = mk(res.UB, res.line, 0.07, 0.07, 1, 0, 0, 0, root);
    lineVk = mk(res.UB, res.line, 0.07, 0.07, 1, 0, 0, 0, root);

    // Anchors for labels.
    const at = (parent, x, y2, z) => { const a = new THREE.Object3D(); a.position.set(x, y2, z); parent.add(a); return a; };
    anchors = {
      aSeal: at(E.seal, 0, 0.7, 0),
      aSeal2: at(E2.seal, 0, 0.5, 0),
      tok0: E.tokens[0], tok1: E.tokens[1], tok2: E.tokens[2], collar: E.collar,
      tab: E.tab, strip1: E.strip, cstrip: at(E.group, CALL_AT[0], 0.35, CALL_AT[1]),
      rack, door, dust, vk: vkCard.userData.anchor, rackCard: at(rack, 0, -0.2, 0.1), doorFront: at(door, 0, 1.5, 2.2),
      housing: at(row, -12, 0.4, ROW_Z), tapeRow: at(row, -5.5, 0.15, ROW_Z), sockets: at(E.group, -0.35, 0.6, SOCK_Z),
      changed: E2.changed, refused: E2.refused, envTop: at(E.group, 1.8, 0.8, 1.8),
    };
    Object.entries(anchors).forEach(([k, v]) => ctx.labels.anchor(k, v));

    const s = ctx.camera.shots;
    ctx.camera.track([
      { t: 0, pose: s.establish([-10, 1, 0], 30), dur: 2 },
      { t: 9, pose: s.inspect([-9, 0, 0], 9, 'top'), dur: 1.5 },
      { t: 32, pose: s.inspect([-9, 0, 2.0], 13, 'top'), dur: 1.5 },
      { t: 55, pose: s.establish([1, 1, 0], 36), dur: 1.5 },
      { t: 82, pose: s.follow([-8, 0.8, -1.6], [-3, 11, 16]), dur: 1.5 },
      { t: 88, pose: s.follow([20, 0.3, 0], [-5, 8, 15]), dur: 4 },
    ]);
  },

  seek(t) {
    const b = ctx.beatAt(t);
    const bi = b.index;
    const bt = b.t;
    const out = ctx.plates.out;

    for (const [n, c] of Object.entries(CONTENT)) ctx.carrier.show(n, 1, c);

    // ---- envelope on the bench
    let ex = ENV_X;
    if (bi === 3) ex = lerp(ENV_X, out.x, phase(bt, 6, 10));
    const hop = 0.16 * (1 - smooth(Math.abs(ex), 3.0, 4.5));
    const rise = 0.2 * smooth(ex, 13.5, 15.4);
    E.group.position.set(ex, hop + rise, 0);

    // ---- seal slides in from the in plate
    const sp = bi === 0 ? phase(bt, T.arrive.sealA, T.arrive.sealB) : 1;
    E.seal.position.set(
      lerp(ctx.plates.in.x - ENV_X, SEAL_AT[0], sp),
      lerp(ctx.plates.in.y, SEAL_AT[1], sp) + 0.9 * Math.sin(PI * sp),
      lerp(0, SEAL_AT[2], sp),
    );

    // ---- values leave the carrier and seat
    for (let i = 0; i < 4; i++) {
      const obj = i < 3 ? E.tokens[i] : E.collar;
      const k = bi === 0 ? phase(bt, T.arrive.tok0 + T.arrive.tokStep * i, T.arrive.tok0 + T.arrive.tokStep * i + T.arrive.tokDur) : 1;
      const started = bi > 0 || bt >= T.arrive.tok0 + T.arrive.tokStep * i;
      obj.visible = started;
      const y0 = i < 3 ? TOKEN_Y : 0.43;
      obj.position.set(lerp(0, SOCK_X[i], k), lerp(0.4, y0, k) + 0.6 * Math.sin(PI * k), lerp(5.8, SOCK_Z, k));
      obj.scale.setScalar(i < 3 ? lerp(0.5, 1, k) : lerp(0.5, 1, k));
      if (i === 3) obj.scale.set(0.56 * lerp(0.5, 1, k), 0.14 * lerp(0.5, 1, k), 0.56 * lerp(0.5, 1, k));
    }
    // program token moves to the call-data position in beat 3
    let kp = 0;
    if (bi === 2) kp = phase(bt, 2, 3.2);
    else if (bi >= 3) kp = 1;
    const p0 = E.tokens[0];
    if (bi >= 2) {
      p0.position.set(
        lerp(SOCK_X[0], CALL_AT[0], kp),
        lerp(TOKEN_Y, CALL_Y, kp) + 0.6 * Math.sin(PI * kp),
        lerp(SOCK_Z, CALL_AT[1], kp),
      );
    }

    // ---- locking bar, its teeth, the sweep and the tab
    const lockX = bi === 0 ? lerp(-0.3, 0, phase(bt, T.arrive.lockA, T.arrive.lockB)) : 0;
    E.lock.position.x = lockX;
    const t1x = bi >= 2 ? p0.position.x - lockX : SOCK_X[0];
    const t1z = bi >= 2 ? Math.max(-0.15, Math.min(0.15, p0.position.z - 0.85)) : 0.15;
    setTeeth(E.teeth, t1x, t1z);
    const sv = bi === 0 ? ramp(bt, T.arrive.lockB, T.arrive.sweepB) : 0;
    E.sweep.visible = sv > 0 && sv < 1;
    E.sweep.position.x = lerp(-2.2, 2.2, sv);
    const tabLit = bi === 0 ? ramp(bt, T.arrive.sweepB, T.arrive.tabB) : 1;
    res.tabMat.emissiveIntensity = 0.5 * tabLit;

    // ---- comparison envelope (beat 2 only)
    let s2 = 0;
    if (bi === 1) s2 = phase(bt, 0.4, 0.8);
    else if (bi === 2) s2 = 1 - phase(bt, 0, 0.6);
    E2.group.visible = s2 > 0.001;
    E2.group.scale.setScalar(Math.max(s2, 0.001));
    E2.group.position.y = lerp(0.15, 0, s2);
    let l2 = -0.3;
    let hz = 0;
    let tk = 0;
    if (bi === 1) {
      l2 = bt < 2.3 ? lerp(-0.3, -0.12, phase(bt, 2, 2.3)) : lerp(-0.12, -0.2, phase(bt, 2.3, 2.6));
      hz = phase(bt, 2.8, 3.4);
      tk = phase(bt, 3.0, 3.4);
    }
    E2.lock.position.x = l2;
    setTeeth(E2.teeth, SOCK_X[0], 0.15);
    E2.hatch.visible = hz > 0.001;
    E2.hatch.scale.setScalar(Math.max(hz, 0.001));
    segment([SOCK_X[2], 0.3, SOCK_Z + 0.38], [SOCK_X[2], 0.3, SOCK_Z + 0.65], tk, E2.tick);

    // ---- rack, door and connectors
    const ra = bi < 2 ? 0 : bi === 2 ? phase(bt, 0, 0.4) : 1;
    const lit = bi < 2 ? 0 : bi === 2 ? phase(bt, 0.6, 1.2) : 1;
    for (const o of [rack, door, vkCard]) { o.visible = ra > 0.001; o.scale.setScalar(Math.max(ra, 0.001)); }
    res.cards.emissiveIntensity = 0.45 * lit;
    res.doorRim.emissiveIntensity = 0.45 * lit;
    const ck = bi === 2 ? phase(bt, 4, 5) : bi === 3 ? 1 - phase(bt, 0, 1.4) : 0;
    const wx = E.group.position.x;
    segment([wx + CALL_AT[0], 0.55, CALL_AT[1]], [DOOR[0], LINT_Y, 0], ck, lineData);
    VK_STATE.lit = lit;
    vkCard.userData.setState(VK_STATE);
    segment([VK[0], VK[1] - 0.45, VK[2] + 0.1], [DOOR[0], LINT_Y + 0.3, -1.4], ck, lineVk);

    // ---- row that stays off-chain and the stop bar (beat 4)
    const rw = bi === 3 ? phase(bt, 0.4, 0.8) : 0;
    row.visible = rw > 0.001;
    row.scale.set(1, Math.max(rw, 0.001), 1);
    const sb = bi === 3 ? phase(bt, 6.0, 6.6) : 0;
    stopBar.visible = sb > 0.001;
    stopBar.scale.y = 0.5 * Math.max(sb, 0.001);
    stopBar.position.y = 0.25 * sb;

    // ---- fee and lid (beat 4)
    const dv = bi === 3;
    dust.visible = dv;
    if (dv) {
      const da = phase(bt, 0, 0.4);
      const dk = phase(bt, 2, 3.2);
      dust.scale.setScalar(Math.max(da, 0.001));
      dust.position.set(lerp(-5.5 - ENV_X, FEE_AT[0], dk), lerp(1.6, 0.39, dk), lerp(-2.4, FEE_AT[1], dk));
    }
    const la = dv ? phase(bt, 3.0, 3.4) : 0;
    lid.visible = dv && la > 0.001;
    lid.scale.setScalar(Math.max(la, 0.001));
    lid.position.set(0, LID_Y + 1.0 * (1 - phase(bt, 3.6, 4.2)), 0);
  },

  dispose() {
    if (root) {
      root.traverse((o) => {
        if (o.geometry) o.geometry.dispose();
        if (o.material) (Array.isArray(o.material) ? o.material : [o.material]).forEach((m) => m.dispose());
      });
      ctx.scene.remove(root);
    }
    ctx = root = res = E = E2 = rack = door = dust = lid = lineData = lineTick = lineVk = vkCard = row = stopBar = null;
    anchors = {};
  },
};
