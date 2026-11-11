// Scene time state. Everything the scene draws is a function of T alone: phases are taken in
// absolute scene seconds from the start of a beat, so a finished motion stays finished.
import { phase, ramp, lerp, clamp01 } from '../../theme.js';
import { BEATS } from './beats.js';

export const START = [];
let acc = 0;
for (const b of BEATS) { START.push(acc); acc += b.duration; }
export const TOTAL = acc;

export const T = { t: 0, b: 0, tb: 0 };
export function setTime(t, b) {
  T.t = t; T.b = b.index; T.tb = b.t;
}
// Eased phase of beat i between local seconds a and b.
export const ph = (i, a, b) => phase(T.t, START[i] + a, START[i] + b);
// Linear ramp.
export const rp = (i, a, b) => ramp(T.t, START[i] + a, START[i] + b);
// Seconds into beat i (can be negative or beyond its length).
export const since = (i, a = 0) => T.t - START[i] - a;
// True from beat i, local second a, on.
export const after = (i, a = 0) => T.t >= START[i] + a;
// A window: 1 between a and b (local seconds of beat i), with 0.4 s scale ramps at both ends.
export const win = (i, a, b, r = 0.4) => clamp01((T.t - START[i] - a) / r) * clamp01((START[i] + b - T.t) / r);
export const mix = lerp;

// The plan under the lid is drawn only while the lid is lifted.
export const planOn = () => !(T.b === 0 || (T.b === 1 && T.tb < 2.0) || T.b >= 32 || (T.b === 31 && after(31, 3.4)));
// Tree scale: shown from the lid lift of beat 1 until the lid closes in beat 31.
export const treeK = () => {
  if (T.b === 0) return 0;
  const a = rp(1, 3.1, 3.5);
  return T.b >= 31 ? a * (1 - rp(31, 0, 0.4)) : a;
};
