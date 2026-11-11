// Scene 8: The network. Six Midnight nodes verify with one key, the accepted claim becomes a fact
// on chain, collections settle as [reg_root, set_root, shape_d], and the camera pulls back over
// all eight stations. Every pose is a pure function of the scene time (see seek).
import * as THREE from 'three/webgpu';
import { PALETTE, FAMILY, materials, makeDigestToken, phase, ramp, lerp, clamp01 } from '../theme.js';
import { ident } from '../labels.js';
import { makeClaimPlate, makeProofSlab, makeVerifierKeyCard } from '../kit.js';
import { BEAT_TEXT, INVENTORY } from './scene8/text.js';

const HALF = Math.PI / 2;
const NODE_X = [0, 1, 2, 3, 4, 5].map((i) => -12 + 3.6 * i);
const NODE_Z = [-3.6, -4.0, -4.2, -4.2, -4.0, -3.6];
const SWEEP_AT = [8.0, 8.15, 8.3, 8.1, 8.25, 8.05];
const REC = { x: -1.2, y: 0.6, z: 2.05 };
const SOCK = [[-5.9, 3.3], [-4.7, 3.3], [-3.5, 3.3], [-2.3, 3.3]];
const PORT = [[3.2, 1.4], [3.2, 3.4]];
const CART_X = [-5.25, -3.75, -2.25, -0.75];
const C1 = [10, 0, 1.4];
const C2 = [11, 0, -2.6];
const PIN_X = [12.2, 13.2, 14.2];
const SEAL = [13.2, -2.4];
const POST = [9.4, -2.4];
const TRACE_X0 = -300;
const TRACE_X1 = 20;
const A_CON = [0.5, 0.95, 2.05];
const TRAY = { x: 11.2, z: -0.8, w: 8.8, d: 9.0, top: 0.1 };

const ln = (x, y, z) => new THREE.Vector3(x, y, z);
const _o = new THREE.Object3D();
const _c = new THREE.Color();
const _v = new THREE.Vector3();
const ZERO = new THREE.Vector3();

const S = { ctx: null };

const L = (id, text, anchor, kind, dy, from, to) => ({ id, text, anchor, kind, dy, ...(from != null ? { from } : {}), ...(to != null ? { to } : {}) });

const LABELS = [
  [
    L('nodes', 'Midnight nodes', 'nodesTop', 'neutral', 0.3),
    L('ledger', 'Ledger 9.1.0.1', 'station', 'neutral', 0.3),
    L('key', 'Decider verifier key', 'card4', 'state', 0.5, 8.6),
    L('indep', 'Independent verification', 'tracks', 'neutral', 0.3, 7.0),
    L('stored', 'Stored claim', 'record', 'outcome', 1.2),
  ],
  [
    L('seal', 'Decider proof', 'sealOpen', 'state', 0.3),
    L('kb', 'About 6.4 KB', 'annot', 'state', 0.3),
    L('key', 'Decider verifier key', 'card2', 'state', 0.5),
    L('tape', 'Witness tape', 'tapeFar', 'neutral', 0.9),
  ],
  [
    L('prog', ident('prog_d'), 'tProg', 'program', 0.7, 4.6),
    L('inp', ident('input_d'), 'tInput', 'input', 0.7, 4.6),
    L('out', ident('outcome_d'), 'storedOut', 'outcome', 1.5),
    L('steps', ident('n_steps'), 'storedSteps', 'state', 1.5),
    L('claim', ident('claim_d'), 'claimFront', 'state', 0.5),
    L('acc', 'Accepted claim', 'plinth', 'state', 0.5, 7.0),
  ],
  [
    L('join', 'JOIN', 'relation', 'neutral', 0.5),
    L('dom', ident('SHK_SET_JOIN'), 'domain', 'state', 0.6, 0.6, 4.0),
    L('left', 'Left collection', 'setLb', 'state', 0),
    L('right', 'Right collection', 'setRb', 'state', 0),
    L('root', ident('set_root'), 'setLRb', 'state', 0, 5.4),
    L('open', 'Membership openings', 'pathAn', 'neutral', 0),
    L('off', 'Off-chain', 'trayCorner', 'zoneoff', 0, 0.6),
  ],
  [
    L('graph', 'Proof graph', 'graph', 'neutral', 0.5),
    L('dep', 'Shared dependency', 'dep', 'state', 0.5),
    L('desc', 'Authenticated descriptors', 'descTray', 'state', 0.4, 3.2, 6.0),
    L('shape', ident('shape_d'), 'tShape', 'state', 0.9, 6.0),
    L('off', 'Off-chain', 'trayCorner', 'zoneoff', 0),
  ],
  [
    L('stmt', 'Collection statement', 'sealStrip', 'state', 0.5),
    L('reg', ident('reg_root'), 'tReg', 'state', 0.6, 3.6),
    L('set', ident('set_root'), 'tSet', 'state', 1.5, 3.6),
    L('shape', ident('shape_d'), 'tShape', 'state', 0.6, 3.6),
    L('creg', 'Contract registry', 'postTok', 'state', 0.7),
  ],
  [
    L('roots', 'Collection roots', 'g1set', 'state', 0.6),
    L('shapes', 'Shape digests', 'g2shape', 'state', 0.6),
    L('claim', ident('claim_d'), 'ringClaim', 'state', 0.7, 3.6),
    L('shared', 'Shared claim', 'record', 'state', 1.2),
  ],
  [
    L('prog', ident('prog_d'), 'tProg', 'program', 0.7, null, 5.0),
    L('inp', ident('input_d'), 'tInput', 'input', 0.7, null, 5.0),
    L('out', ident('outcome_d'), 'storedOut', 'outcome', 1.5, null, 5.0),
    L('steps', ident('n_steps'), 'storedSteps', 'state', 1.5, null, 5.0),
    L('claim', ident('claim_d'), 'claimFront', 'state', 0.5, null, 5.0),
    L('acc', 'Accepted claim', 'plinth', 'state', 0.6),
  ],
  [
    L('acc', 'Accepted claim', 'plinth', 'state', 0.6),
  ],
];

const DURATIONS = [26, 26, 22, 25, 25, 27, 24, 32, 56];
const STARTS = DURATIONS.map((d, i) => DURATIONS.slice(0, i).reduce((a, b) => a + b, 0));

// ---- helpers ---------------------------------------------------------------------------------
const park = (g, k) => { g.visible = k > 0.001; g.scale.setScalar(Math.max(k, 0.001)); };
const putW = (o, x, y, z) => {
  const p = o.parent === S.ctx.scene ? ZERO : o.parent.position;
  // Children of a group carried by colTray keep their height above the tray surface.
  o.position.set(x - p.x, o.parent.userData.carried ? y : y - p.y, z - p.z);
};
const flatY = (size, surface) => surface + 0.09 * size + 0.004;
const arc = (k, h) => Math.sin(Math.PI * clamp01(k)) * h;

function setBar(m, a, b, k, w, h) {
  const dx = b[0] - a[0], dy = b[1] - a[1], dz = b[2] - a[2];
  const len = Math.hypot(dx, dy, dz);
  const kk = clamp01(k);
  m.visible = kk > 0.001;
  m.position.set(a[0] + (dx * kk) / 2, a[1] + (dy * kk) / 2, a[2] + (dz * kk) / 2);
  m.rotation.order = 'YZX';
  m.rotation.set(0, -Math.atan2(dz, dx), Math.atan2(dy, Math.hypot(dx, dz)));
  m.scale.set(Math.max(len * kk, 1e-4), h, w);
}
function instBar(im, idx, a, b, w, h) {
  const dx = b[0] - a[0], dy = b[1] - a[1], dz = b[2] - a[2];
  _o.rotation.order = 'YZX';
  _o.position.set((a[0] + b[0]) / 2, (a[1] + b[1]) / 2, (a[2] + b[2]) / 2);
  _o.rotation.set(0, -Math.atan2(dz, dx), Math.atan2(dy, Math.hypot(dx, dz)));
  _o.scale.set(Math.hypot(dx, dy, dz), h, w);
  _o.updateMatrix();
  im.setMatrixAt(idx, _o.matrix);
}
function instBox(im, idx, x, y, z, sx, sy, sz) {
  _o.rotation.set(0, 0, 0);
  _o.position.set(x, y, z);
  _o.scale.set(sx, sy, sz);
  _o.updateMatrix();
  im.setMatrixAt(idx, _o.matrix);
}

// One hash motion (STORYBOARD 0.5): u seconds since it starts. Returns the block emissive.
function hashRun(h, u) {
  const n = h.ins.length, dt = Math.min(0.3, 1.2 / n);
  for (let q = 0; q < n; q++) {
    const it = h.ins[q];
    const m = phase(u, 0, 0.6);
    const sk = phase(u, 0.6 + dt * q, 0.6 + dt * (q + 1));
    const x = lerp(lerp(it.from[0], it.file[0], m), h.slotIn[0], sk);
    const y = lerp(lerp(it.from[1], it.file[1], m), h.slotIn[1], sk);
    const z = lerp(lerp(it.from[2], it.file[2], m), h.slotIn[2], sk);
    putW(it.o, x, y, z);
    it.o.visible = sk < 0.999;
    it.o.scale.setScalar(Math.max(it.s * (1 - sk), 0.001));
  }
  const ok = phase(u, 2.2, 3.0);
  const o = h.out;
  o.visible = u > 2.2;
  putW(o, lerp(h.slotOut[0], h.slotOut[0] + 1.6, ok), h.slotOut[1], h.slotOut[2]);
  o.scale.setScalar(Math.max(h.outSize * lerp(0.2, 1, ok), 0.001));
  return u > 1.8 && u < 2.2 ? 0.8 * (1 - Math.abs((u - 1.8) / 0.2 - 1)) : 0;
}

export default {
  id: 'network',
  title: 'The network',
  beats: BEAT_TEXT.map((b, i) => ({ id: b.id, title: b.title, duration: DURATIONS[i], narration: b.narration, where: [0, 1, 2, 5, 6, 7, 8].includes(i) ? 'on' : 'off', labels: LABELS[i], ...(i === 8 ? { inventory: INVENTORY } : {}) })),

  build(c) {
    const ctx = c;
    S.ctx = ctx;
    S.geos = [];
    S.mats = [];
    S.kit = [];
    const scene = ctx.scene;
    const G = (g) => { S.geos.push(g); return g; };
    const M = (m) => { S.mats.push(m); return m; };

    const unit = G(new THREE.BoxGeometry(1, 1, 1));
    const hexGeo = G(new THREE.CylinderGeometry(0.5, 0.5, 1, 6));
    const torusGeo = G(new THREE.TorusGeometry(1, 0.07, 8, 32));

    const matSlate = M(materials.slate());
    const matTi = M(materials.titanium());
    const matCer = M(materials.ceramic());
    const matInk = M(materials.ink());
    const matGold = M(materials.gold());
    const matHash = M(materials.gold());
    matHash.emissive = new THREE.Color(PALETTE.goldBright);
    matHash.emissiveIntensity = 0;
    const matPaper = M(materials.paper());
    const matLine = M(new THREE.MeshBasicMaterial({ color: PALETTE.onSurface }));
    const matSweep = M(new THREE.MeshBasicMaterial({ color: PALETTE.goldBright }));
    const matTrace = M(new THREE.MeshBasicMaterial({ color: PALETTE.gold }));
    const matEnv = M(materials.slate());
    matEnv.emissive = new THREE.Color(FAMILY.step.bright);
    matEnv.emissiveIntensity = 0;
    const matRule = M(materials.family('step'));
    matRule.emissive = new THREE.Color(FAMILY.step.bright);
    matRule.emissiveIntensity = 0;
    const matCap = M(materials.gold());
    matCap.emissive = new THREE.Color(PALETTE.goldBright);
    matCap.emissiveIntensity = 0;
    const matEdge = M(materials.gold());
    matEdge.emissive = new THREE.Color(PALETTE.goldBright);
    matEdge.emissiveIntensity = 0;
    const matFlash = M(materials.gold());
    matFlash.emissive = new THREE.Color(PALETTE.goldBright);
    matFlash.emissiveIntensity = 1;
    // One ring material: the three ring flashes (8.3, 8.6, 8.7) never overlap in time.
    const matRing = M(materials.gold());
    matRing.emissive = new THREE.Color(PALETTE.goldBright);
    matRing.emissiveIntensity = 0;
    const ringMats = [matRing, matRing, matRing];
    Object.assign(S, { matEnv, matRule, matCap, matEdge, matHash, ringMats, matGold, matTi });

    const add = (parent, geo, mat, sx, sy, sz, x, y, z) => {
      const m = new THREE.Mesh(geo, mat);
      m.scale.set(sx, sy, sz);
      m.position.set(x, y, z);
      parent.add(m);
      return m;
    };
    const group = (parent, x = 0, y = 0, z = 0) => {
      const g = new THREE.Group();
      g.position.set(x, y, z);
      parent.add(g);
      return g;
    };
    const anchor = (name, parent, x, y, z) => {
      const o = new THREE.Object3D();
      o.position.set(x, y, z);
      parent.add(o);
      ctx.labels.anchor(name, o);
      return o;
    };

    // ---- digest tokens with shared geometry and materials ------------------------------------
    const share = (g) => g;   // token geometry, rim, cap and faces are shared in theme.js
    const tok = (parent, family, content, size) => {
      const g = share(makeDigestToken({ family, content, size: 1 }));
      g.scale.setScalar(size);
      g.visible = false;
      parent.add(g);
      return g;
    };

    // ---- nodes -------------------------------------------------------------------------------
    const nodes = group(scene);
    const plinthIM = new THREE.InstancedMesh(unit, matSlate, 6);
    const capIM = new THREE.InstancedMesh(unit, matTi, 6);
    const postIM = new THREE.InstancedMesh(unit, matTi, 6);
    for (let k = 0; k < 6; k++) {
      instBox(plinthIM, k, NODE_X[k], 0.35, NODE_Z[k], 2.6, 0.7, 1.8);
      instBox(capIM, k, NODE_X[k], 0.75, NODE_Z[k], 2.7, 0.1, 1.9);
      instBox(postIM, k, NODE_X[k], 0.75, NODE_Z[k] - 1.0, 0.16, 1.5, 0.16);
    }
    nodes.add(plinthIM, capIM, postIM);
    S.envs = [];
    S.rims = [];
    S.cards = [];
    S.sweeps = [];
    for (let k = 0; k < 6; k++) {
      const env = group(nodes, NODE_X[k], 0.8, NODE_Z[k] + 0.15);
      env.scale.setScalar(0.3);
      const rimMat = M(materials.titanium());
      add(env, unit, rimMat, 4.8, 0.16, 3.4, 0, 0.08, 0);
      add(env, unit, matEnv, 4.2, 0.8, 2.8, 0, 0.56, 0);
      add(env, unit, matCer, 2.4, 0.05, 1.4, 0, 0.985, 0);
      S.envs.push(env);
      S.rims.push(rimMat);
      // Decider verifier key card (AMEND-1 section 1.4): no gold proving key on the on-chain side.
      const card = makeVerifierKeyCard({ size: 0.95 });
      card.position.set(NODE_X[k], 1.85, NODE_Z[k] - 0.92);
      nodes.add(card);
      S.kit.push(card);
      S.cards.push(card);
      const sw = add(nodes, unit, matSweep, 0.05, 0.03, 0.95, NODE_X[k], 1.2, NODE_Z[k] + 0.15);
      sw.visible = false;
      S.sweeps.push(sw);
    }
    anchor('nodesTop', scene, -7, 3.0, -4.3);
    anchor('station', scene, 4.0, 0.4, -1.0);
    anchor('tracks', scene, NODE_X[1], 0.9, NODE_Z[1] + 1.0);
    ctx.labels.anchor('card4', S.cards[4]);
    ctx.labels.anchor('card2', S.cards[2]);

    // ---- open envelope on node 3 (beat 8.2) ---------------------------------------------------
    const X2 = NODE_X[2], Z2 = NODE_Z[2];
    const gOpen = group(scene, X2, 0.8, Z2);
    add(gOpen, unit, matSlate, 2.52, 0.1, 1.7, 0, 0.05, 0);
    add(gOpen, unit, matTi, 2.52, 0.28, 0.08, 0, 0.24, -0.81);
    add(gOpen, unit, matTi, 2.52, 0.28, 0.08, 0, 0.24, 0.81);
    add(gOpen, unit, matTi, 0.08, 0.28, 1.7, -1.22, 0.24, 0);
    add(gOpen, unit, matTi, 0.08, 0.28, 1.7, 1.22, 0.24, 0);
    const SOCK_C = ['prog:main', 'input:main', 'outcome:main', 'n_steps:main'];
    const SOCK_F = ['program', 'input', 'outcome', 'state'];
    SOCK_C.forEach((cc, j) => {
      add(gOpen, unit, matInk, 0.38, 0.02, 0.38, -0.75 + 0.5 * j, 0.11, 0.55);
      const t = tok(gOpen, SOCK_F[j], cc, 0.3);
      t.rotation.x = -HALF;
      t.position.set(-0.75 + 0.5 * j, flatY(0.3, 0.12), 0.55);
      t.visible = true;
    });
    S.calL = [add(gOpen, unit, matTi, 0.07, 0.5, 0.07, -1, 0.35, -0.15), add(gOpen, unit, matTi, 0.3, 0.05, 0.07, -0.9, 0.58, -0.15)];
    S.calR = [add(gOpen, unit, matTi, 0.07, 0.5, 0.07, 1, 0.35, -0.15), add(gOpen, unit, matTi, 0.3, 0.05, 0.07, 0.9, 0.58, -0.15)];
    add(gOpen, unit, matLine, 1.41, 0.02, 0.03, 0, 0.78, -0.15);
    add(gOpen, unit, matLine, 0.03, 0.14, 0.03, -0.705, 0.78, -0.15);
    add(gOpen, unit, matLine, 0.03, 0.14, 0.03, 0.705, 0.78, -0.15);
    S.openSweep = add(gOpen, unit, matSweep, 0.05, 0.03, 1.5, 0, 0.5, 0);
    anchor('annot', gOpen, 0, 0.78, -0.15);
    anchor('sealOpen', gOpen, 0, 0.3, -0.15);
    S.gOpen = gOpen;

    // Kit seal: four pins in the open envelope (8.2), three pins at the collection (8.6).
    const seal = makeProofSlab({ variant: 'decider', scale: 1, pinKinds: ['capcollar', 'capcollar', 'capcollar', 'capcollar'] });
    scene.add(seal);
    S.seal = seal;
    S.kit.push(seal);

    // ---- far tape tray (8.2) -----------------------------------------------------------------
    const gTape = group(scene, -13, 0, 5.2);
    add(gTape, unit, matPaper, 5, 0.1, 1.5, 0, 0.05, 0);
    const tapeCubes = new THREE.InstancedMesh(unit, matCer, 18);
    const tapePlates = new THREE.InstancedMesh(unit, matCer, 3);
    const tapeRules = new THREE.InstancedMesh(unit, matRule, 3);
    [-1.6, -0.5, 1.6].forEach((fx, f) => {
      instBox(tapePlates, f, fx, 0.13, 0, 0.54, 0.036, 0.72);
      instBox(tapeRules, f, fx, 0.19, 0, 0.18, 0.05, 0.3);
      for (let q = 0; q < 3; q++) {
        instBox(tapeCubes, f * 6 + q, fx - 0.2, 0.19, (q - 1) * 0.18, 0.1, 0.1, 0.1);
        instBox(tapeCubes, f * 6 + 3 + q, fx + 0.2, 0.19, (q - 1) * 0.18, 0.1, 0.1, 0.1);
      }
    });
    gTape.add(tapeCubes, tapePlates, tapeRules);
    add(gTape, unit, matLine, 0.5, 0.03, 0.05, 0.55 + 0.1, 0.12, 0.12).rotation.y = 0.35;
    add(gTape, unit, matLine, 0.5, 0.03, 0.05, 0.55 - 0.1, 0.12, -0.12).rotation.y = 0.35;
    anchor('tapeFar', gTape, 0, 0.3, 0);
    S.gTape = gTape;
    S.tapeLink = add(scene, unit, matLine, 1, 0.04, 0.06, 0, 0.06, 0);
    S.tapeBreak = [add(scene, unit, matLine, 0.5, 0.04, 0.05, 0, 0.07, 0), add(scene, unit, matLine, 0.5, 0.04, 0.05, 0, 0.07, 0)];

    // ---- record (drawer) ---------------------------------------------------------------------
    const record = group(scene);
    add(record, unit, matSlate, 3.2, 0.12, 1.7, 0, 0.06, 0);
    add(record, unit, matSlate, 3.2, 0.9, 0.1, 0, 0.45, -0.8);
    add(record, unit, matSlate, 0.1, 0.9, 1.7, -1.55, 0.45, 0);
    add(record, unit, matSlate, 0.1, 0.9, 1.7, 1.55, 0.45, 0);
    add(record, unit, matSlate, 3.2, 0.9, 0.1, 0, 0.45, 0.8);
    add(record, unit, matTi, 3.3, 0.05, 0.18, 0, 0.9, 0.8);
    add(record, unit, matTi, 0.18, 0.05, 1.8, -1.55, 0.9, 0);
    add(record, unit, matTi, 0.18, 0.05, 1.8, 1.55, 0.9, 0);
    add(record, unit, matTi, 3.3, 0.05, 0.18, 0, 0.9, -0.8);
    const plate = makeClaimPlate({ size: 0.7 });
    plate.position.set(0, 0.12, -0.05);
    record.add(plate);
    S.kit.push(plate);
    share(plate.userData.claimToken);
    share(plate.userData.outcomeToken);
    share(plate.userData.stepsToken);
    const claimFront = tok(record, 'state', 'claim:main', 0.8);
    claimFront.visible = true;
    claimFront.position.set(0, 0.45, 0.88);
    S.record = record;
    S.plate = plate;
    ctx.labels.anchor('record', record);
    ctx.labels.anchor('claimFront', claimFront);
    ctx.labels.anchor('storedOut', plate.userData.outcomeToken);
    ctx.labels.anchor('storedSteps', plate.userData.stepsToken);

    // ---- claim plinth, sockets, ports --------------------------------------------------------
    const plinth = group(scene, -3, 0, 2.4);
    add(plinth, unit, matSlate, 7, 0.52, 2.6, 0, 0.26, 0);
    add(plinth, unit, matTi, 7.1, 0.08, 2.7, 0, 0.56, 0);
    ctx.labels.anchor('plinth', plinth);
    SOCK.forEach(([sx, sz], j) => {
      if (j < 3) add(scene, hexGeo, matInk, 0.56, 0.04, 0.56, sx, 0.62, sz).rotation.y = HALF / 3;
      else add(scene, unit, matInk, 1.0, 0.04, 0.7, sx, 0.62, sz);
    });
    S.ports = PORT.map(([px, pz]) => {
      add(scene, unit, matTi, 0.4, 1.2, 0.4, px, 0.6, pz);
      add(scene, unit, matCap, 0.5, 0.1, 0.5, px, 1.25, pz);
    });
    S.conLine = [add(scene, unit, matGold, 1, 0.08, 0.1, 0, 0, 0), add(scene, unit, matGold, 1, 0.08, 0.1, 0, 0, 0)];

    // carried tokens (8.3)
    S.cart = [
      tok(scene, 'program', 'prog:main', 0.9), tok(scene, 'input', 'input:main', 0.9),
      tok(scene, 'outcome', 'outcome:main', 0.9), tok(scene, 'state', 'n_steps:main', 0.9),
    ];
    ctx.labels.anchor('tProg', S.cart[0]);
    ctx.labels.anchor('tInput', S.cart[1]);
    const ring = (mat, parent) => {
      const r = new THREE.Mesh(torusGeo, mat);
      r.visible = false;
      parent.add(r);
      return r;
    };
    S.ringOut = ring(ringMats[0], scene);
    S.ringSteps = ring(ringMats[0], scene);
    S.ringClaim = ring(ringMats[2], scene);
    S.sweep = add(scene, unit, matSweep, 0.07, 0.03, 2.5, 0, 0.66, 2.4);
    S.sweep.visible = false;
    ctx.labels.anchor('ringClaim', S.ringClaim);

    // ---- off-chain tray colTray (8.4, 8.5; AMEND-1 section 2) ---------------------------------
    const colTray = group(scene, TRAY.x, 0.05, TRAY.z);
    const matZoneOff = M(new THREE.MeshStandardMaterial({ color: PALETTE.zoneOff, roughness: 0.6, metalness: 0.1 }));
    add(colTray, unit, matPaper, TRAY.w, 0.1, TRAY.d, 0, 0, 0);
    const RIM = 0.14, RH = 0.12, ry = 0.05 + RH / 2;
    add(colTray, unit, matZoneOff, TRAY.w, RH, RIM, 0, ry, -(TRAY.d - RIM) / 2);
    add(colTray, unit, matZoneOff, TRAY.w, RH, RIM, 0, ry, (TRAY.d - RIM) / 2);
    add(colTray, unit, matZoneOff, RIM, RH, TRAY.d - 2 * RIM, -(TRAY.w - RIM) / 2, ry, 0);
    add(colTray, unit, matZoneOff, RIM, RH, TRAY.d - 2 * RIM, (TRAY.w - RIM) / 2, ry, 0);
    anchor('trayCorner', colTray, -TRAY.w / 2 + 0.6, 0.2, -TRAY.d / 2 + 0.7);
    S.colTray = colTray;

    // ---- join (8.4) -------------------------------------------------------------------------
    const gC1 = group(scene, ...C1);
    gC1.userData.carried = true;
    S.gC1 = gC1;
    const hashBlock = (parent, x, z) => {
      const g = group(parent);
      add(g, unit, matHash, 2.4, 1.0, 1.2, 0, 0, 0);
      add(g, unit, matInk, 0.1, 0.5, 0.6, -1.2, 0, 0);
      add(g, unit, matInk, 0.1, 0.5, 0.6, 1.2, 0, 0);
      putW(g, x, 0.5, z);
      return g;
    };
    S.hash1 = hashBlock(gC1, 10.5, 0.4);
    S.node1Edge = add(gC1, unit, matGold, 3.14, 0.1, 2.14, 0, 0, 0);
    putW(S.node1Edge, 10.5, 0.05, 2.6);
    S.node1Body = add(gC1, unit, matTi, 3.0, 0.3, 2.0, 0, 0, 0);
    putW(S.node1Body, 10.5, 0.25, 2.6);
    anchor('relation', scene, 10.5, 1.0, -0.2);
    S.setL = tok(gC1, 'state', 'set:l', 0.8);
    S.setR = tok(gC1, 'state', 'set:r', 0.8);
    S.copyL = tok(gC1, 'state', 'set:l', 0.8);
    S.copyR = tok(gC1, 'state', 'set:r', 0.8);
    S.setLR = tok(gC1, 'state', 'set:lr', 1.2);
    S.domain = group(gC1);
    add(S.domain, unit, matGold, 0.45, 0.45, 0.45, 0, 0, 0);
    ctx.labels.anchor('domain', S.domain);
    anchor('setLb', scene, 9.45, 0.5, 1.5);
    anchor('setRb', scene, 11.55, 0.5, 1.5);
    anchor('setLRb', scene, 10.5, 0.5, 1.5);
    anchor('pathAn', scene, 12.4, 0.5, 2.9);
    const mkPath = (name) => {
      const p = group(scene);
      add(p, unit, matLine, 0.05, 0.03, 0.7, 0, 0.03, 0.8);
      add(p, unit, matGold, 0.22, 0.22, 0.22, 0, 0.12, 0.62);
      add(p, unit, matGold, 0.22, 0.22, 0.22, 0, 0.12, 0.92);
      const t = tok(p, 'state', name, 0.5);
      t.rotation.x = -HALF;
      t.position.set(0, flatY(0.5, 0), 1.35);
      t.visible = true;
      return p;
    };
    S.pathL = mkPath('claim:a');
    S.pathR = mkPath('claim:b');
    ctx.labels.anchor('pathR', S.pathR);

    // ---- mini graphs ---------------------------------------------------------------------------
    const miniGraph = (parent, slabs, links, extra = []) => {
      const body = new THREE.InstancedMesh(unit, matTi, slabs.length);
      const edge = new THREE.InstancedMesh(unit, matEdge, slabs.length);
      slabs.forEach(([x, z], k) => {
        instBox(body, k, x, 0.075, z, 0.9, 0.09, 0.6);
        instBox(edge, k, x, 0.015, z, 0.942, 0.03, 0.642);
      });
      const segs = [];
      links.forEach(([a, b]) => {
        const A = slabs[a], B = slabs[b];
        const dx = B[0] - A[0], dz = B[1] - A[1];
        if (Math.abs(dz) > Math.abs(dx)) {
          const sg = Math.sign(dz);
          segs.push([[A[0], 0.1, A[1] + sg * 0.3], [B[0], 0.1, B[1] - sg * 0.3]]);
        } else {
          const sg = Math.sign(dx);
          segs.push([[A[0] + sg * 0.45, 0.1, A[1]], [B[0] - sg * 0.45, 0.1, B[1]]]);
        }
      });
      extra.forEach((s) => segs.push(s));
      const bars = new THREE.InstancedMesh(unit, matLine, segs.length);
      segs.forEach((s, k) => instBar(bars, k, s[0], s[1], 0.05, 0.03));
      parent.add(body, edge, bars);
      return segs;
    };
    const rel = (arr, o) => arr.map((p) => [p[0] - o[0], p[1] - o[2]]);
    const gC2 = group(scene, ...C2);
    gC2.userData.carried = true;
    S.gC2 = gC2;
    const GP = [[8.2, -2.0], [10.0, -2.0], [12.2, -2.0], [14.0, -2.0], [9.1, -3.3], [13.1, -3.3], [11.1, -4.6]];
    S.graphPos = GP;
    const gs = rel(GP, C2);
    const brk = (a, b) => {
      const mx = (a[0] + b[0]) / 2, mz = (a[2] + b[2]) / 2, ang = Math.atan2(b[2] - a[2], b[0] - a[0]);
      const out = [];
      for (const d of [-0.1, 0.1]) {
        const cx = mx + Math.cos(ang) * d, cz = mz + Math.sin(ang) * d;
        const t = ang + 1.22;
        out.push([[cx - Math.cos(t) * 0.2, 0.1, cz - Math.sin(t) * 0.2], [cx + Math.cos(t) * 0.2, 0.1, cz + Math.sin(t) * 0.2]]);
      }
      return out;
    };
    const depAt = [11.1 - C2[0], -2.0 - C2[2]];
    const extra = [
      [[depAt[0] - 0.25, 0.1, depAt[1]], [gs[1][0] + 0.45, 0.1, gs[1][1]]],
      [[depAt[0] + 0.25, 0.1, depAt[1]], [gs[2][0] - 0.45, 0.1, gs[2][1]]],
    ];
    const rootC = [gs[6][0], 0.1, gs[6][1] + 0.3];
    const j0 = [gs[4][0], 0.1, gs[4][1] - 0.3];
    const j1 = [gs[5][0], 0.1, gs[5][1] - 0.3];
    extra.push(...brk(j0, rootC), ...brk(j1, rootC));
    miniGraph(gC2, gs, [[0, 4], [1, 4], [2, 5], [3, 5], [4, 6], [5, 6]], extra);
    S.dep = tok(gC2, 'state', 'claim:dep', 0.5);
    S.dep.rotation.x = -HALF;
    S.dep.position.set(depAt[0], flatY(0.5, 0), depAt[1]);
    S.dep.visible = true;
    ctx.labels.anchor('dep', S.dep);
    S.hash2 = hashBlock(gC2, 11, -0.4);
    S.descTray = add(gC2, unit, matTi, 2.5, 0.05, 0.6, 0, 0, 0);
    putW(S.descTray, 8.25, 0.025, -0.4);
    ctx.labels.anchor('descTray', S.descTray);
    const gAnch = anchor('graph', gC2, 0, 0.1, 0);
    putW(gAnch, 11.1, 0.1, -3.3);
    S.cells = [0, 1, 2, 3, 4].map(() => add(scene, unit, matCer, 0.3, 0.3, 0.3, 0, 0, 0));
    S.cells.forEach((m) => { m.visible = false; });
    S.tShape = tok(scene, 'state', 'shape:graph', 1.2);
    ctx.labels.anchor('tShape', S.tShape);

    // ---- collection settlement (8.6) ---------------------------------------------------------
    S.tReg = tok(scene, 'state', 'reg:root', 1.2);
    S.tSet = tok(scene, 'state', 'set:lr', 1.2);
    ctx.labels.anchor('tReg', S.tReg);
    ctx.labels.anchor('tSet', S.tSet);
    S.regCopy = tok(scene, 'state', 'reg:root', 0.8);
    const gPost = group(scene, POST[0], 0, POST[1]);
    add(gPost, unit, matSlate, 0.5, 0.8, 0.5, 0, 0.4, 0);
    add(gPost, unit, matTi, 0.56, 0.06, 0.56, 0, 0.83, 0);
    S.postTok = tok(gPost, 'state', 'reg:root', 0.8);
    S.postTok.position.set(0, 1.3, 0);
    S.postTok.visible = true;
    S.gPost = gPost;
    ctx.labels.anchor('postTok', S.postTok);
    S.ringReg = ring(ringMats[1], gPost);
    S.ringReg.position.set(0, 1.3, 0);
    S.ringReg.scale.setScalar(0.5);
    anchor('sealStrip', scene, SEAL[0] - 1.4, 0.4, SEAL[1] - 0.9);

    // ---- two small graphs (8.7) --------------------------------------------------------------
    const mkSmall = (z0, chain, tag) => {
      const g = group(scene, 5.5, 0, z0);
      const pos = chain ? [[-0.8, 0], [0.4, 0], [1.6, 0]] : [[-0.6, 0], [0.9, -0.7], [0.9, 0.7]];
      miniGraph(g, pos, chain ? [[0, 1], [1, 2]] : [[0, 1], [0, 2]]);
      const a = tok(g, 'state', `set:${tag}`, 0.7);
      a.rotation.x = -HALF; a.position.set(-0.2, flatY(0.7, 0), -1.4); a.visible = true;
      const b = tok(g, 'state', `shape:${tag}`, 0.7);
      b.rotation.x = -HALF; b.position.set(1.0, flatY(0.7, 0), -1.4); b.visible = true;
      const m = tok(g, 'state', 'claim:main', 0.8);
      m.rotation.x = -HALF; m.position.set(2.7, flatY(0.8, 0), 0); m.visible = true;
      ctx.labels.anchor(`${tag}set`, a);
      ctx.labels.anchor(`${tag}shape`, b);
      return { g, member: m };
    };
    const g1 = mkSmall(-0.6, true, 'g1');
    const g2 = mkSmall(4.4, false, 'g2');
    S.g1 = g1; S.g2 = g2;

    // ---- trace and flash rings (8.8) -------------------------------------------------------
    S.trace = add(scene, unit, matTrace, 1, 0.04, 0.12, 0, 0.03, 5.6);
    S.trace.visible = false;
    S.flashRings = [];
    for (let k = 0; k < 8; k++) {
      const r = new THREE.Mesh(torusGeo, matFlash);
      r.rotation.x = -HALF;
      r.scale.set(2.3, 2.3, 2.3);
      r.position.set(20 - 40 * k, 2.2, 0);
      r.visible = false;
      scene.add(r);
      S.flashRings.push(r);
    }

    // Static poses used as travel targets.
    record.position.set(REC.x, REC.y, REC.z);
    scene.updateMatrixWorld(true);
    const sceneW = scene.getWorldPosition(new THREE.Vector3());
    const local = (o) => o.getWorldPosition(new THREE.Vector3()).sub(sceneW);
    S.tgtOut = local(plate.userData.outcomeToken);
    S.tgtSteps = local(plate.userData.stepsToken);
    S.tgtClaim = local(claimFront);
    S.ringOut.position.copy(S.tgtOut);
    S.ringOut.rotation.x = -HALF;
    S.ringOut.scale.setScalar(0.35 * 0.62);
    S.ringSteps.position.copy(S.tgtSteps);
    S.ringSteps.rotation.x = -HALF;
    S.ringSteps.scale.setScalar(0.35 * 0.62);
    S.ringClaim.position.copy(S.tgtClaim);
    S.ringClaim.scale.setScalar(0.8 * 0.62);

    // ---- camera ------------------------------------------------------------------------------
    const s = ctx.camera.shots;
    // The pull-back pose of 8.8, held through 8.9: the bench sits in the upper part of the frame, above
    // the inventory card of 8.9 (lower 62 percent of the stage).
    const W0 = s.establish([-140, 0, 0], 400);
    const WIDE = { ...W0, pos: [W0.pos[0], W0.pos[1] - 85, W0.pos[2]], target: [W0.target[0], W0.target[1] - 85, W0.target[2]] };
    const B = STARTS;
    // Beat 8.9 adds no key: it holds the WIDE pose that 8.8 ends on.
    ctx.camera.track([
      { t: 0, pose: s.establish([-10, 1, 0], 30), dur: 2 },
      { t: 6, pose: s.establish([-3, 0.8, -2.5], 24), dur: 1.5 },
      { t: B[1], pose: s.establish([-4.8, 0.8, -4.0], 7), dur: 1.5 },
      { t: B[2], pose: s.establish([-1, 0.6, 2.4], 13), dur: 1.5 },
      { t: B[3], pose: s.inspect([11, 0, -0.6], 14.5, 'top'), dur: 1.5 },
      { t: B[4], pose: s.inspect([11, 0, -2.0], 12, 'top'), dur: 1.5 },
      { t: B[5], pose: s.establish([11.5, 0.6, -0.5], 13), dur: 1.5 },
      { t: B[6], pose: s.establish([2, 0.6, 1.5], 22), dur: 1.5 },
      { t: B[7], pose: s.establish([-1, 0.6, 2.4], 13), dur: 1.5 },
      { t: B[7] + 3, pose: WIDE, dur: 5 },
    ]);
  },

  seek(t) {
    const ctx = S.ctx;
    const b = ctx.beatAt(t);
    const i = b.index, bt = b.t;
    const P = (k, a, c) => (i < k ? 0 : i > k ? 1 : phase(bt, a, c));
    const R = (k, a, c) => (i < k ? 0 : i > k ? 1 : ramp(bt, a, c));
    const pulse = (k, a, c) => (i === k && bt > a && bt < c ? 1 - Math.abs(((bt - a) / (c - a)) * 2 - 1) : 0);
    const box = (k, a, c) => (i === k && bt >= a && bt < c ? 1 : 0);

    // Carrier: all four from 8.3 on go to zero while the scene tokens take over.
    if (i >= 2) {
      const lower = i > 2 ? 1 : phase(bt, 2, 3);
      for (const n of ['prog_d', 'input_d', 'outcome_d', 'n_steps']) ctx.carrier.show(n, 1 - lower);
    }

    // ---- record ----------------------------------------------------------------------------
    {
      const k = P(0, 2, 4);
      S.record.position.set(lerp(-20, REC.x, k), lerp(0.2, REC.y, k) + arc(k, 0.5), lerp(0, REC.z, k));
      const lit = pulse(2, 5.6, 5.9);
      S.plate.userData.setState({ claim: 0, stored: 1, lit });
    }

    // ---- nodes -----------------------------------------------------------------------------
    const envLit = P(0, 6, 7.5);
    S.matEnv.emissiveIntensity = 0.3 * envLit;
    const openK = P(1, 0, 0.8) - P(2, 0, 0.8);
    for (let k = 0; k < 6; k++) {
      const s0 = SWEEP_AT[k];
      const sw = i === 0 && bt > s0 && bt < s0 + 0.4 ? (bt - s0) / 0.4 : 0;
      const sweep = S.sweeps[k];
      sweep.visible = sw > 0;
      if (sw > 0) sweep.position.set(NODE_X[k] + lerp(-0.7, 0.7, sw), 1.2, NODE_Z[k] + 0.15);
      const g = i > 0 ? 1 : ramp(bt, s0 + 0.4, s0 + 0.5);
      S.rims[k].color.setHex(PALETTE.titanium).lerp(_c.setHex(PALETTE.gold), g);
      S.cards[k].userData.setState({ lit: i === 0 && bt >= s0 + 0.4 && bt < s0 + 0.7 ? 1 : 0 });
      S.envs[k].scale.setScalar(k === 2 ? 0.3 * Math.max(1 - openK, 0.001) : 0.3);
      S.envs[k].visible = k !== 2 || openK < 0.999;
    }

    // ---- open envelope, seal and caliper (8.2) ------------------------------------------------
    park(S.gOpen, openK);
    const seal = S.seal;
    const sealOpen = openK > 0.001;
    const collK = P(5, 0.6, 1.0) * (1 - P(6, 0, 0.4));
    const cal = P(1, 2, 3);
    S.calL.forEach((m, j) => { m.position.x = -lerp(1.15, 0.8, cal) + (j ? 0.15 : 0); });
    S.calR.forEach((m, j) => { m.position.x = lerp(1.15, 0.8, cal) - (j ? 0.15 : 0); });
    const osw = i === 1 && bt > 3 && bt < 3.4 ? (bt - 3) / 0.4 : 0;
    S.openSweep.visible = osw > 0;
    S.openSweep.position.x = lerp(-0.85, 0.85, osw);
    const pins = seal.userData.pins;
    if (i === 1) {
      seal.position.set(NODE_X[2], 0.9, NODE_Z[2] - 0.15);
      seal.scale.setScalar(0.45 * Math.max(openK, 0.001));
      seal.visible = sealOpen;
      pins.forEach((p, j) => { p.visible = true; p.position.x = -1.3 + (j * 2.6) / 3; });
      seal.userData.setState({ pins: [], verify: 0, edgeLit: pulse(1, 3.4, 3.7), ring: { show: 1, folds: 12, closed: 1 } });
    } else {
      seal.position.set(SEAL[0], 0, SEAL[1]);
      seal.scale.setScalar(Math.max(collK, 0.001));
      seal.visible = collK > 0.001;
      pins.forEach((p, j) => { p.visible = j < 3; p.position.x = j - 1; });
      const v = R(5, 5.8, 6.2);
      seal.userData.setState({
        pins: ['state', 'state', 'state'],
        verify: i === 5 && bt < 6.2 ? v : 0,
        edgeLit: pulse(5, 6.2, 6.5),
        ring: { show: 0, folds: 0, closed: 0 },
      });
    }

    // ---- far tape and its connector -------------------------------------------------------
    const tapeK = P(1, 0, 0.8) - P(2, 0, 0.4);
    park(S.gTape, tapeK);
    S.matRule.emissiveIntensity = 0.6 * (i === 1 ? phase(bt, 0.8, 1.6) : i > 1 ? 1 : 0);
    {
      const a = [-10.9, 0.1, 4.6], e = [-5.7, 0.1, -2.3];
      const k = tapeK > 0.001 ? P(1, 0.8, 2.4) : 0;
      setBar(S.tapeLink, a, e, k, 0.06, 0.04);
      const mx = (a[0] + e[0]) / 2, mz = (a[2] + e[2]) / 2;
      const ang = Math.atan2(e[2] - a[2], e[0] - a[0]);
      S.tapeBreak.forEach((m, j) => {
        const d = (j ? 1 : -1) * 0.12;
        m.position.set(mx + Math.cos(ang) * d, 0.07, mz + Math.sin(ang) * d);
        m.rotation.set(0, -(ang + 1.22), 0);
        m.visible = k > 0.55 && tapeK > 0.5;
      });
      S.tapeLink.visible = S.tapeLink.visible && tapeK > 0.001;
    }

    // ---- claim: tokens, sockets, merge, read lines (8.3) ---------------------------------------
    {
      const rise = P(2, 2, 3);
      const seat = P(2, 3, 4.6);
      const merge = R(2, 3, 4.2);
      for (let j = 0; j < 4; j++) {
        const T = S.cart[j];
        const sx = CART_X[j];
        const shown = i > 2 || (i === 2 && bt > 2);
        if (j < 2) {
          T.visible = shown;
          const y0 = lerp(0.1, 0.55, rise);
          const sc = 0.9 * lerp(0.2, 1, rise);
          const tx = SOCK[j][0], tz = SOCK[j][1], ty = flatY(0.9, 0.64);
          T.position.set(lerp(sx, tx, seat), lerp(y0, ty, seat) + arc(seat, 0.8), lerp(5.6, tz, seat));
          T.rotation.set(-HALF * seat, 0, 0);
          T.scale.setScalar(sc);
        } else {
          const tgt = j === 2 ? S.tgtOut : S.tgtSteps;
          T.visible = i === 2 && bt > 2 && bt < 4.0;
          const y0 = lerp(0.1, 0.55, rise);
          const sc = lerp(0.9 * lerp(0.2, 1, rise), 0.35, merge);
          T.position.set(lerp(sx, tgt.x, merge), lerp(y0, tgt.y, merge) + arc(merge, 1.0), lerp(5.6, tgt.z, merge));
          T.rotation.set(-HALF * merge, 0, 0);
          T.scale.setScalar(sc);
        }
      }
      const rOn = i > 2 || (i === 2 && bt >= 4.0);
      S.ringOut.visible = rOn;
      S.ringSteps.visible = rOn;
    }
    {
      const k = i === 2 && bt > 5.2 && bt < 5.6 ? (bt - 5.2) / 0.4 : 0;
      S.sweep.visible = k > 0;
      S.sweep.position.x = lerp(-6.4, 0.4, k);
      const kc = R(2, 6, 7);
      setBar(S.conLine[0], A_CON, [3.0, 0.95, PORT[0][1]], kc, 0.1, 0.08);
      setBar(S.conLine[1], A_CON, [3.0, 0.95, PORT[1][1]], kc, 0.1, 0.08);
      S.matCap.emissiveIntensity = 0.6 * R(2, 7, 7.3);
    }

    // ---- off-chain tray: rises at the start of 8.4, lowers and hides at the start of 8.6 ---------
    const trayK = P(3, 0, 0.6) * (1 - P(5, 0, 0.6));
    const lift = TRAY.top * trayK;
    S.colTray.visible = trayK > 0.001;
    S.colTray.position.y = lerp(-0.2, 0.05, trayK);
    S.gC1.position.y = lift;
    S.gC2.position.y = lift;

    // ---- join (8.4) -------------------------------------------------------------------------
    const col1 = P(3, 0, 0.6) * (1 - P(4, 0, 0.5));
    park(S.gC1, col1);
    if (!S.H1) {
      S.H1 = { slotIn: [9.3, 0.5, 0.4], slotOut: [11.7, 0.5, 0.4], out: S.setLR, outSize: 1.2, ins: [
        { o: S.domain, from: [7.4, 0.25, -1.3], file: [8.2, 0.5, 0.4], s: 1 },
        { o: S.copyL, from: [9.45, 0.476, 2.05], file: [7.3, 0.5, 0.4], s: 0.8 },
        { o: S.copyR, from: [11.55, 0.476, 2.05], file: [6.4, 0.5, 0.4], s: 0.8 }] };
      S.H2 = { slotIn: [9.8, 0.5, -0.4], slotOut: [12.2, 0.5, -0.4], out: S.tShape, outSize: 1.2, ins: S.cells.map((o, q) => ({
        o, from: [9.25 - 0.55 * q, 0.5, -0.4], file: [9.25 - 0.55 * q, 0.5, -0.4], s: 1 })) };
    }
    let hashPulse = 0;
    {
      for (const o of [S.setL, S.setR, S.copyL, S.copyR, S.setLR]) o.rotation.x = -HALF;
      S.setL.visible = true; S.setR.visible = true;
      putW(S.setL, 9.45, flatY(0.8, 0.4), 2.05);
      putW(S.setR, 11.55, flatY(0.8, 0.4), 2.05);
      S.setL.scale.setScalar(0.8); S.setR.scale.setScalar(0.8);
      const u = i === 3 ? bt - 2 : i > 3 ? 99 : -1;
      if (u < 0) {
        S.domain.visible = true;
        S.domain.scale.setScalar(Math.max(P(3, 0, 0.4), 0.001));
        putW(S.domain, 7.4, 0.25, -1.3);
        S.copyL.visible = false; S.copyR.visible = false; S.setLR.visible = false;
      } else {
        hashPulse = hashRun(S.H1, u);
        const ks = P(3, 5, 7);
        putW(S.setLR, lerp(13.3, 10.5, ks), lerp(0.5, flatY(1.2, 0.4), ks) + arc(ks, 0.8), lerp(0.4, 2.15, ks));
      }
      const p8 = i < 5;
      const pk = p8 ? col1 : P(5, 0.6, 1.4) * (1 - P(6, 0, 0.4));
      const pos = p8 ? [9.45, 0.4, 2.05, 11.55] : [12.4, 0, 0.2, 14.0];
      S.pathL.position.set(pos[0], pos[1] + (p8 ? lift : 0), pos[2]);
      S.pathR.position.set(pos[3], pos[1] + (p8 ? lift : 0), pos[2]);
      for (const p of [S.pathL, S.pathR]) park(p, pk * (p8 ? 1 : 0.7));
    }

    // ---- proof graph (8.5) ------------------------------------------------------------------
    {
      park(S.gC2, P(4, 0, 0.6) * (1 - P(5, 0, 0.5)));
      S.matEdge.emissiveIntensity = i === 4 ? 0.5 * phase(bt, 0, 1) : 0;
      const GP = S.graphPos;
      const src = [6, 4, 5, 1, 2];
      const u = i === 4 ? bt - 3 : -1;
      if (i === 4 && u >= 0) {
        hashPulse = hashRun(S.H2, u);
        const kd = P(4, 6, 7);
        S.tShape.position.y = lerp(0.5, flatY(1.2, 0), kd) + lift;
      } else if (i === 4) {
        const kl = phase(bt, 2, 3);
        S.cells.forEach((o, q) => {
          o.visible = bt >= 2;
          o.scale.setScalar(1);
          const f = S.H2.ins[q].file;
          putW(o, lerp(GP[src[q]][0], f[0], kl), lerp(0.3, f[1], kl) + arc(kl, 0.5) + lift, lerp(GP[src[q]][1], f[2], kl));
        });
        S.tShape.visible = false;
      } else {
        S.cells.forEach((o) => { o.visible = false; });
        S.tShape.visible = i > 4;
      }
      S.tShape.rotation.x = -HALF;
      S.matHash.emissiveIntensity = hashPulse;
    }

    // ---- settle (8.6) -----------------------------------------------------------------------
    {
      const mv = P(5, 2, 3.4);
      const fade = 1 - P(6, 0, 0.4);
      const toks = [S.tReg, S.tSet, S.tShape];
      const st = [[6.6, -1.0], [8.4, -1.0], [13.8, -0.4]];
      toks.forEach((T, j) => {
        if (i < 5 || (j === 2 && i === 4)) { if (j < 2) T.visible = false; return; }
        const ap = j < 2 ? P(5, 0.6, 1.0) : 1;
        T.visible = fade * ap > 0.001;
        T.position.set(lerp(st[j][0], PIN_X[j], mv), lerp(flatY(1.2, 0) + lift, 0.55, mv) + arc(mv, 1.0), lerp(st[j][1], SEAL[1] + 1.5, mv));
        T.rotation.set(-HALF * (1 - mv), 0, 0);
        T.scale.setScalar(Math.max(1.2 * lerp(1, 0.8 / 1.2, mv) * ap * fade, 0.001));
      });
      const rk = ramp(bt, 4, 5.2);
      S.regCopy.visible = i === 5 && bt >= 4.0 && bt < 5.0;
      S.regCopy.position.set(lerp(PIN_X[0], POST[0], rk), lerp(0.55, 1.3, rk), lerp(SEAL[1] + 1.5, POST[1], rk));
      S.regCopy.scale.setScalar(0.8);
      park(S.gPost, P(5, 0.6, 1.0) * fade);
      S.ringReg.visible = i > 5 || (i === 5 && bt >= 5.0);
    }

    // ---- claim identity (8.7) ---------------------------------------------------------------
    {
      const gk = P(6, 0, 0.5) * (1 - P(7, 6, 8));
      const mv = P(6, 2, 3.4);
      [[S.g1, -0.6], [S.g2, 4.4]].forEach(([g, z0]) => {
        park(g.g, gk);
        const m = g.member;
        m.visible = i < 6 ? true : i === 6 ? bt < 3.2 : false;
        if (i === 6 || i < 6) {
          putW(m, lerp(8.2, S.tgtClaim.x, mv), lerp(flatY(0.8, 0), S.tgtClaim.y, mv) + arc(mv, 0.8), lerp(z0, S.tgtClaim.z, mv));
          m.rotation.x = -HALF * (1 - mv);
        }
      });
      S.ringClaim.visible = i > 6 || (i === 6 && bt >= 3.2);
      S.ringMats[0].emissiveIntensity = 0.5 * (box(2, 4.0, 4.3) + box(5, 5.0, 5.3) + box(6, 3.2, 3.5));
    }

    // ---- the whole path (8.8) ---------------------------------------------------------------
    {
      if ((i === 7 && bt >= 3) || i === 8) ctx.showAll(true);
      const len = (TRACE_X1 - TRACE_X0) * R(7, 8, 14);
      const w = lerp(0.12, 1.4, P(7, 3, 8));
      const th = lerp(0.04, 0.2, P(7, 3, 8));
      S.trace.visible = len > 0.01;
      S.trace.position.set(TRACE_X0 + len / 2, th / 2, 5.6);
      S.trace.scale.set(Math.max(len, 0.001), th, w);
      // From the end of the pull-back the fine detail is below one pixel: it leaves the draw list.
      const fine = !(i > 7 || (i === 7 && bt >= 8));
      S.envs.forEach((e, k) => { if (k !== 2) e.visible = fine; });
      S.cards.forEach((cd) => { cd.visible = fine; });
      S.cart[0].visible = S.cart[0].visible && fine;
      S.cart[1].visible = S.cart[1].visible && fine;
      S.plate.visible = fine;
      S.flashRings.forEach((r, k) => {
        const tk = 8 + 6 * ((20 - 40 * k - TRACE_X0) / (TRACE_X1 - TRACE_X0));
        r.visible = i === 7 && bt >= tk && bt < tk + 0.3;
      });
    }

    // ---- what lives where (8.9): the wide pose holds; the two rails and the gate light in turn --------
    if (i === 8) {
      const win = (a) => phase(bt, a, a + 0.25) * (1 - phase(bt, a + 0.95, a + 1.2));
      ctx.zone.glow('off', win(2.0));
      ctx.zone.glow('gate', win(6.0));
      ctx.zone.glow('on', win(10.0));
    }
  },

  dispose() {
    if (S.ctx) S.ctx.scene.traverse((o) => { if (o.isInstancedMesh) o.dispose(); });
    (S.kit || []).forEach((k) => k.userData.dispose());
    (S.geos || []).forEach((g) => g.dispose());
    (S.mats || []).forEach((m) => m.dispose());
    for (const k of Object.keys(S)) delete S[k];
    S.ctx = null;
  },
};
