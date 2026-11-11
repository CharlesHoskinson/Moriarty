// CORE, DEC, RANGE and ALU (beats 3.2, 3.3, 3.6 to 3.9) with their beat-driven overlays.
import { PALETTE, FAMILY, lerp } from '../../theme.js';
import * as D from './draw.js';
import { T, ph, rp, after, win } from './time.js';
import { lit } from './lit.js';
import { eq, lerp3, sweepGlow } from './motion.js';
import { pointAt } from './route.js';

export const regX = (i) => -10.75 + 0.5 * i;
export const REG_Z = [-1.6, -1.0];
export const slotX = (i) => -11 + 0.4 * i;
export const CHAIN_X = [-11, -9.5, -8, -6.5, -5];
export const CHAIN_Z = 1.9;
export const COND = { x: -4.9, z: -1.7 };
export const READ = [[-4.9, 0.3], [-4.9, 0.8], [-4.9, 1.3]];
const WRITTEN = [2, 5];
const P = [0, 0, 0];
const ROUTE = [[0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0]];
const set = (r, x, y, z) => { r[0] = x; r[1] = y; r[2] = z; return r; };

// DEC rows (z), table rows 0 to 7 with a break between 3 and 4, then the idle row.
const DEC_ROW_Z = [-4.2, -4.06, -3.92, -3.78, -3.5, -3.36, -3.22, -3.08, -2.94];
const DEC_X = -0.2;
export const TABLE_Z = -4.65;

export function drawCore() {
  const bi = T.b;
  D.dim(1 - lit('core'));
  D.housing(-8, 0, 7, 5);
  // Registers: before row and after row, eight cells each.
  const wk = ph(1, 13.5, 13.9);
  for (let i = 0; i < 8; i++) {
    D.cell(regX(i), 0.3, REG_Z[0], 0.4);
    const w = WRITTEN.includes(i) && wk > 0.5;
    D.cell(regX(i), 0.3, REG_Z[1], 0.4, w ? PALETTE.heat : PALETTE.ceramic);
  }
  if (wk > 0.01) for (let i = 0; i < 8; i++) if (!WRITTEN.includes(i)) D.outline(regX(i), 0.52, REG_Z[1], 0.4, 0.4, PALETTE.onSurface, 0.03);
  // Rule slots.
  for (let i = 0; i < 16; i++) D.cell(slotX(i), 0.3, -2.2, 0.3, bi >= 2 && i === 3 ? PALETTE.ceramicEdge : PALETTE.ceramic);
  // Append chain: five cubes joined by a line.
  D.line(CHAIN_X[0], CHAIN_Z, CHAIN_X[4], CHAIN_Z, 1, 0.33);
  for (let i = 0; i < 5; i++) D.cell(CHAIN_X[i], 0.3, CHAIN_Z, 0.4);
  // Read port and bus port.
  for (let i = 0; i < 3; i++) D.cell(READ[i][0], 0.3, READ[i][1], 0.4);
  D.box('ti', -4.7, 0.5, -0.6, 0.4, 0.4, 1.0, PALETTE.titanium);
  D.cell(COND.x, 0.3, COND.z, 0.4, bi === 7 && T.tb > 6.4 ? PALETTE.ceramic : PALETTE.ceramic);

  // Beat 3.2: the after row of slot 3 (a copy, lifted 0.3) moves onto the before row of slot 4.
  if (bi === 1) {
    const e = eq(1, 11.5);
    const ap = rp(1, 10.8, 11.3);
    if (ap > 0.01 && e.show) {
      const z = lerp(REG_Z[1], REG_Z[0], e.m);
      const y = lerp(0.6, 0.3, e.m);
      for (let i = 0; i < 8; i++) D.cell(regX(i), y, z, 0.4, PALETTE.ceramic, ap);
    }
    if (e.ring) D.ring(-9.0, 0.5, REG_Z[0], 3.9, 0.4, e.lit);
  }
  D.dim(0);
}

// DEC, its table and the dispatch overlay (beat 3.3).
export function drawDec() {
  const bi = T.b;
  D.dim(1 - lit('dec'));
  D.housing(0.5, -3.6, 3, 1.5);
  const sel = ph(2, 4.0, 4.4);
  for (let r = 0; r < 9; r++) {
    const raised = r === 2 && bi >= 2 ? 0.15 * sel : 0;
    const on = r === 2 && bi >= 2 && sel > 0.5;
    const idle = r === 8;
    for (let c = 0; c < 5; c++) {
      const x = DEC_X + (c - 2) * 0.28;
      const col = on ? PALETTE.heat : idle ? PALETTE.ceramicEdge : PALETTE.ceramic;
      const dimmed = bi === 2 && sel > 0 && r !== 2;
      if (dimmed) D.dim(Math.max(1 - lit('dec'), 0.7 * sel));
      D.box('matte', x, 0.34 + raised, DEC_ROW_Z[r], 0.22, 0.08, 0.1, col);
      if (dimmed) D.dim(1 - lit('dec'));
    }
  }
  D.breakMark(DEC_X, 0.34, -3.64, 0.9);
  // Table rack with the two tag cells.
  D.box('ti', 2.4, 0.1, TABLE_Z, 7, 0.1, 0.3, PALETTE.titanium);
  const tagLit = bi === 8 ? ph(8, 7.0, 8.2) : 0;
  D.fcell(-0.8, 0.15, TABLE_Z, 0.28, 'step', tagLit, PALETTE.ceramic);
  D.fcell(5.6, 0.15, TABLE_Z, 0.28, 'step', tagLit, PALETTE.ceramic);
  // Line at z -3.6 between CORE and DEC.
  D.line(-9.8, -3.6, -1.0, -3.6, 1, 0.04);

  if (bi === 2) {
    // Dispatch cells slide from three before cells to the left edge of DEC.
    for (let i = 0; i < 3; i++) {
      const k = ph(2, 2.0, 3.0);
      const ez = -3.6 + (i - 1) * 0.18;
      const x = regX(i);
      let px; let pz;
      if (k < 0.5) { const u = k * 2; px = x; pz = lerp(REG_Z[0], -3.6, u); } else { const u = (k - 0.5) * 2; px = lerp(x, -1.2, u); pz = lerp(-3.6, ez, u); }
      if (k > 0.001) D.cell(px, 0.35, pz, 0.28);
    }
    // Sweep down the table, then the returning control word and its connectors.
    D.sweepZ(-4.28, -2.9, 0.42, DEC_X, 1.5, rp(2, 3.4, 3.4 + 1.0));
  }
  if (bi >= 2) {
    if (bi > 2 || T.tb > 4.0) {
      // Dispatch cells rest at the DEC edge.
      for (let i = 0; i < 3; i++) D.cell(-1.2, 0.35, -3.6 + (i - 1) * 0.18, 0.28);
    }
    const rk = ph(2, 5.0, 6.4);
    if (rk > 0.001) {
      ROUTE[0] = set(ROUTE[0], DEC_X, 0.55, DEC_ROW_Z[2]);
      ROUTE[1] = set(ROUTE[1], -1.2, 0.55, -3.6);
      ROUTE[2] = set(ROUTE[2], slotX(3), 0.55, -3.6);
      ROUTE[3] = set(ROUTE[3], slotX(3), 0.55, -2.2);
      pointAt(ROUTE, rk, P);
      for (let c = 0; c < 5; c++) D.cell(P[0] + (c - 2) * 0.22, P[1] - 0.1, P[2], 0.2, PALETTE.heat);
      if (rk >= 1) {
        for (const w of WRITTEN) D.line(slotX(3), -2.2, regX(w), REG_Z[1], 1, 0.62);
      }
    }
  }
  D.dim(0);
}

// RANGE housing and limb column (beat 3.9).
const LIMB_Z = (j) => -3.6 + (j - 5) * 0.12;
export function drawRange() {
  const bi = T.b;
  D.dim(1 - lit('range'));
  D.housing(4.3, -3.6, 3, 1.5);
  for (let j = 0; j < 11; j++) D.outline(3.6, 0.31, LIMB_Z(j), 0.14, 0.1, PALETTE.onSurface, 0.02);
  if (bi === 8) {
    const t = T.tb;
    const sendK = ph(8, 2.0, 3.0);
    const splitK = ph(8, 3.0, 4.2);
    const closeK = ph(8, 7.0, 7.5);
    const e = eq(8, 7.2);
    // The copy of the checked ALU cell travels to the RANGE edge, then splits into 11 limbs.
    const sx = lerp(ALU_RES.x, 3.0, sendK);
    const sz = lerp(ALU_RES.z, -3.6, sendK);
    if (t >= 2.0 && splitK < 0.01 && t < 7.2) D.cell(sx, 0.3, sz, 0.28, PALETTE.ceramic);
    if (splitK > 0.01 && closeK < 0.99) {
      const swp = rp(8, 4.6, 6.4);
      for (let j = 0; j < 11; j++) {
        const zc = lerp(-3.6, LIMB_Z(j), splitK * (1 - closeK));
        const xc = lerp(3.0, 3.6, splitK);
        const sw = j === 10 ? 0.12 : 0.25;
        const on = swp > 0 && (j + 0.5) / 11 <= swp ? 1 : 0;
        D.lbox('matte', xc, 0.3 + 0.05, zc, 0.4 * sw * (j === 10 ? 1 : 1), 0.1, 0.1, PALETTE.ceramic, FAMILY.state.bright, on * 0.4);
        if (on) D.line(xc, zc, 1.0 + 0.45 * j, TABLE_Z, 1, 0.42, PALETTE.onSurface, 0.025);
      }
    }
    if (closeK > 0.01 && e.show) {
      const m = e.m;
      D.cell(lerp(3.3, ALU_RES.x, m), 0.3, lerp(-3.6, ALU_RES.z, m), 0.28, PALETTE.ceramic, 1);
    }
    D.sweepZ(-4.2, -3.0, 0.45, 3.6, 0.5, rp(8, 4.6, 6.4));
  }
  D.dim(0);
}

export const ALU_RES = { x: 1.35, z: -2.1 };
const OP_A = 0.55;
const OP_B = 0.95;
const ALU_Z = -2.1;

// ALU (beats 3.6 to 3.8) and the overflow pad.
export function drawAlu() {
  const bi = T.b;
  D.dim(1 - lit('alu'));
  D.housing(0.5, -1.7, 3, 1.5);
  const l7 = bi === 6 ? ph(6, 2.0, 2.4) : 0;
  const sw = bi === 6 ? rp(6, 2.6, 3.4) : 0;
  const swG = bi === 6 && T.tb > 3.4 && T.tb < 3.7 ? 0.8 : 0;
  const fam = bi === 6 || bi === 7;
  D.fcell(OP_A, 0.3, ALU_Z, 0.28, 'step', fam ? Math.max(l7, 0) : 0, PALETTE.ceramic);
  D.fcell(OP_B, 0.3, ALU_Z, 0.28, 'step', fam ? Math.max(l7, 0) : 0, PALETTE.ceramic);
  D.fcell(ALU_RES.x, 0.3, ALU_Z, 0.28, 'step', fam ? Math.max(l7, 0) : 0, PALETTE.ceramic);
  // Flag latch: a plate hinged on its back edge, flat when clear.
  D.push(1.75, 0.34, ALU_Z - 0.2, 0, 0, 0, 1);
  D.box('matte', 0, 0, 0.2, 0.4, 0.05, 0.4, PALETTE.ceramicEdge);
  D.pop();
  if (bi === 6) D.sweepX(OP_A - 0.2, ALU_RES.x + 0.2, 0.5, ALU_Z, 0.5, sw);
  if (bi === 6 || bi === 7) {
    // Connector from the clear latch to the CORE after row.
    const ck = bi === 6 ? ph(6, 3.4, 4.0) : 1;
    D.line(1.75, ALU_Z - 0.3, regX(7), REG_Z[1], ck, 0.62, PALETTE.onSurface, 0.04);
  }
  // Overflow pad with the set latch (beat 3.7), lowered again in beat 3.8.
  const padK = bi === 6 ? ph(6, 6.0, 6.6) : bi === 7 ? 1 - ph(7, 0, 0.8) : 0;
  if (padK > 0.01) {
    D.box('matte', 4.0, 0.05 * padK, -1.6, 2.6 * padK, 0.1 * padK, 1.6 * padK, PALETTE.ceramic);
    D.push(4.0, 0.12, -2.1, 0, 0, 0, padK);
    D.push(0, 0, 0, -Math.PI * (80 / 180), 0, 0, 1);
    D.box('matte', 0, 0.025, 0.2, 0.4, 0.05, 0.4, PALETTE.ceramicEdge);
    D.pop();
    D.pop();
    if (bi === 6) {
      const wk = ph(6, 7.0, 7.8);
      // Payload cell at CORE with the refusal hatch.
      D.line(4.0, -1.9, 4.9, -1.3, wk, 0.4, PALETTE.errorRed, 0.05);
    }
  }
  if (bi === 6 && T.tb > 6.0) {
    const hk = ph(6, 7.8, 8.2);
    D.cell(4.9, 0.1, -1.3, 0.4, PALETTE.ceramic);
    D.hatch(4.9, 0.52, -1.3, hk);
  }
  if (bi === 7) {
    // A second bar settles on the operands and a verification sweep crosses operands and result.
    D.sweepX(OP_A - 0.2, ALU_RES.x + 0.2, 0.5, ALU_Z, 0.5, rp(7, 3.6, 4.3));
    const e = eq(7, 4.4);
    if (T.tb >= 4.4 && e.show) D.cell(lerp(ALU_RES.x, COND.x, e.m), 0.55, lerp(ALU_Z, COND.z, e.m), 0.28, PALETTE.ceramic);
    if (e.ring) D.ring(COND.x, 0.5, COND.z, 0.4, 0.4, e.lit);
  }
  D.dim(0);
}
