// Fixed motions of storyboard section 0.5 as pure functions of scene time.
import { T, ph, rp, START } from './time.js';
import { lerp, clamp01 } from '../../theme.js';

// Equality (1.2 s) starting at local second a of beat i. m: 0..1 move, done: ring on, lit: ring glow.
const EQ = { m: 0, show: 1, ring: 0, lit: 0 };
export function eq(i, a) {
  EQ.m = ph(i, a, a + 1.0);
  const r = rp(i, a + 1.0, a + 1.2);
  EQ.show = r > 0.99 ? 0 : 1;
  EQ.ring = r > 0 ? 1 : 0;
  EQ.lit = r > 0 ? 0.5 * (1 - rp(i, a + 1.2, a + 1.5)) : 0;
  return EQ;
}

// Hash (3.0 s) from local second a of beat i. Returns phases: gather, slide(0..1 over 0.6..1.8), glow, out.
const H = { gather: 0, slide: 0, glow: 0, out: 0 };
export function hashPh(i, a) {
  H.gather = ph(i, a, a + 0.6);
  H.slide = rp(i, a + 0.6, a + 1.8);
  const g = rp(i, a + 1.8, a + 2.2);
  H.glow = g > 0 && g < 1 ? Math.sin(Math.PI * g) : 0;
  H.out = ph(i, a + 2.2, a + 3.0);
  return H;
}

// Verification sweep position 0..1 over [a, b] of beat i (linear), plus edge glow afterwards.
export function sweepPh(i, a, b) {
  return rp(i, a, b);
}
// Edge glow for 0.3 s after a sweep ends.
export function sweepGlow(i, b) {
  const g = rp(i, b, b + 0.3);
  return g > 0 && g < 1 ? 0.8 : 0;
}

// Linear interpolation of a 3-vector into out.
export function lerp3(out, a, b, k) {
  out[0] = lerp(a[0], b[0], k); out[1] = lerp(a[1], b[1], k); out[2] = lerp(a[2], b[2], k);
  return out;
}
export { clamp01, T, START };
