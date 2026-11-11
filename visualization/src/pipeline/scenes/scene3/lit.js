// Region lighting: each beat lights some regions and dims the rest (colour lerp 0.6 toward paperWarm).
import { lerp } from '../../theme.js';
import { T, ph } from './time.js';

const ALL = 'all';
const LIT = [
  ['ribbon'], ['core'], ['core', 'dec'], ['ribbon', 'core'],
  ['bus', 'core', 'alu', 'mem', 'hash', 'page'], ['bus', 'core', 'alu'], ['alu', 'core'], ['alu', 'bus', 'core'],
  ['range', 'alu', 'dec'], ['hash', 'core'], ['core', 'mem', 'bus'], ['mem', 'core'],
  ['mem', 'backend'], ['backend', 'tree'], ['tree'], ['mem', 'backend'],
  ['core', 'mem'], ['tree', 'backend', 'core'], ['page', 'tree', 'backend'], ['page', 'bus', 'core'],
  ['stacks', 'hash'], ['stacks'], ['stacks'], ['vd', 'hash', 'mem', 'core'],
  [], ['bnd', 'core', 'stacks', 'backend'], ['bnd'], ['bnd'],
  ['init', 'core', 'stacks'], ['init', 'stacks', 'core'], ['ribbon'], [],
  [], [], [],
];
const target = (bi, r) => (LIT[bi] === ALL || LIT[bi].includes(r) ? 1 : 0);

// Lit amount of a region at the current time: 0 dim, 1 lit.
export function lit(region) {
  const bi = T.b;
  const prev = target(Math.max(0, bi - 1), region);
  const cur = target(bi, region);
  if (bi === 1) {
    const v = lerp(prev, 1, ph(1, 4.0, 4.8));
    return lerp(v, cur, ph(1, 9.5, 10.3));
  }
  return lerp(prev, cur, ph(bi, 0, 0.7));
}
