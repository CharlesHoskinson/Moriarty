// BUS: two request lanes, the provider lip, branches, chip cells and the request tuples (beats 3.5 to 3.8).
import { PALETTE, FAMILY, lerp } from '../../theme.js';
import * as D from './draw.js';
import { T, ph, rp, after } from './time.js';
import { lit } from './lit.js';
import { eq } from './motion.js';
import { pointAt, routeYaw, busRoute, polyLen, PROVIDER_Z } from './route.js';

export const ROUTE_ALU = [[-4.7, 0.1, -0.6], [-3.3, 0.1, -0.6], [-3.3, 0.1, -1.7], [-1.6, 0.1, -1.7]];
const ROUTE_ALU1 = [[-4.7, 0.1, -0.6], [-2.7, 0.1, -0.6], [-2.7, 0.1, -1.7], [-1.6, 0.1, -1.7]];
const P = [0, 0, 0];
export const BRANCH_X = -1.6;

// Fraction of the route reached at point index n.
function fracAt(route, n) {
  let part = 0;
  for (let i = 1; i <= n; i++) part += Math.hypot(route[i][0] - route[i - 1][0], route[i][2] - route[i - 1][2]);
  return part / polyLen(route);
}
const F_LANE = fracAt(ROUTE_ALU, 1);
const F_JUNC = fracAt(ROUTE_ALU, 2);

// A tuple at fraction f of a route. opts: s, lit, ring, chipFam.
export function tupleOn(route, f, chip, o = {}) {
  pointAt(route, f, P);
  D.tuple(P[0], P[1], P[2], chip, { yaw: routeYaw(route, f), s: o.s ?? 0.7, lit: o.lit ?? 1, k: o.k ?? 1, hex: o.hex });
  return P;
}

export function drawBus() {
  const bi = T.b;
  D.dim(1 - lit('bus'));
  const z0 = -4.3;
  const z1 = 4.6;
  D.line(-3.3, z0, -3.3, z1, 1, 0.05);
  D.line(-2.7, z0, -2.7, z1, 1, 0.05);
  D.line(-2.1, z0, -2.1, z1, 1, 0.05);
  const pz = [PROVIDER_Z.alu, PROVIDER_Z.mem, PROVIDER_Z.hash, PROVIDER_Z.page];
  for (const z of pz) D.line(-2.1, z, -1.0, z, 1, 0.05);

  // Idle zero tuples lie flat in the two lanes.
  const inLane = bi === 4 && T.tb > 2.6 ? 1 : bi > 4 ? 1 : 0;
  if (!inLane) D.tuple(-3.3, 0.0, -0.6, 0, { yaw: -Math.PI / 2, flat: true, chipFam: null, lit: 0, s: 0.7 });
  D.tuple(-2.7, 0.0, -0.6, 0, { yaw: -Math.PI / 2, flat: true, chipFam: null, lit: 0, s: 0.7 });

  // Chip cells on the provider branches (from beat 3.5).
  const ck = ph(4, 3.4, 4.0);
  if (bi >= 4) {
    for (let p = 0; p < 4; p++) {
      D.fcell(BRANCH_X - 0.5, 0.05, pz[p], 0.3, 'step', 0.0, PALETTE.ceramic, ck);
      for (let n = 0; n < p + 1; n++) D.box('matte', BRANCH_X - 0.5 + ((n % 2) - 0.5) * 0.12 * (p > 0 ? 1 : 0), 0.38 * ck, pz[p] + (Math.floor(n / 2) - (p > 1 ? 0.5 : 0)) * 0.12 * (p > 1 ? 1 : 0), 0.06, 0.02, 0.06, PALETTE.ink);
    }
  }

  // Beat 3.5: request tuple rises at the port, enters lane 0 and stops at the ALU junction.
  if (bi === 4) {
    const rise = ph(4, 2.0, 2.6);
    const toLane = ph(4, 2.6, 3.2);
    const toJ = ph(4, 3.6, 4.8);
    const f = toJ > 0 ? lerp(F_LANE, F_JUNC, toJ) : lerp(0, F_LANE, toLane);
    if (rise > 0.01) tupleOn(ROUTE_ALU, f, 1, { k: rise });
  }
  // Beat 3.6: the request goes to the ALU row, equality with the provider tuple, checked tuple returns.
  if (bi === 5) {
    const go = ph(5, 2.0, 3.4);
    const back = ph(5, 6.0, 7.6);
    const e = eq(5, 3.6);
    if (T.tb < 6.0) {
      if (T.tb < 3.6) tupleOn(ROUTE_ALU, lerp(F_JUNC, 1, go), 1);
      else if (e.show) {
        pointAt(ROUTE_ALU, 1, P);
        D.tuple(lerp(P[0], -0.6, e.m), 0.0, P[2], 1, { yaw: 0 });
      }
    } else {
      tupleOn(ROUTE_ALU, lerp(1, 0, back), 1);
      pointAt(ROUTE_ALU, lerp(1, 0, back), P);
      D.ring(P[0], 0.2, P[2], 1.55, 0.3, 0);
    }
    // Provider tuple on the ALU housing at (-0.6, 0.45, -1.7).
    D.tuple(-0.6, 0.3, -1.7, 1, { chipFam: 'step', lit: 0, s: 0.7, hex: PALETTE.ceramic });
    if (e.ring && T.tb < 6.0) D.ring(-0.6, 0.45, -1.7, 1.55, 0.3, e.lit);
  }
  if (bi === 7) {
    // A second request on lane 1 and the ALU branch.
    const go = ph(7, 2.0, 3.2);
    tupleOn(ROUTE_ALU1, go, 1);
  }
  D.dim(0);
}
