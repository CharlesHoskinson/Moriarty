import { createParts } from './scene7/parts.js';
import { Rig } from './scene7/rig.js';
import { buildScript, DUR, T0 } from './scene7/script.js';
import { TEXT } from './scene7/text.js';
import { PLACE } from './scene7/labels.js';

// Scene 7: the contract. Content, timing and labels follow STORYBOARD.md; motion is a pure
// function of scene time through the keyframe rig in scene7/rig.js.
const CARRIER = [['prog_d', 'prog:main'], ['input_d', 'input:main'], ['outcome_d', 'outcome:main'], ['n_steps', 'n_steps:main']];

// Beats whose content fills the lower left: the shell docks the token carrier at the lower right.
const CARRIER_RIGHT = new Set([4, 5, 11]);

const beats = TEXT.map((b, i) => ({
  id: b.id,
  title: b.title,
  duration: DUR[i],
  narration: b.narration,
  where: i <= 5 || i === 20 ? 'on' : 'off',
  carrier: CARRIER_RIGHT.has(i) ? { side: 'right' } : undefined,
  labels: PLACE[i].map((p, j) => {
    if (!p) return null;
    const [where, kind, dy, from, to] = p;
    const l = { id: `l${j}`, text: b.labels[j].text, kind, dy };
    if (typeof where === 'string') l.anchor = where; else l.at = where;
    if (from != null) l.from = from;
    if (to != null) l.to = to;
    return l;
  }).filter(Boolean),
}));

let ctx = null;
let parts = null;
let rig = null;

export default {
  id: 'contract',
  title: 'The contract',
  beats,

  build(c) {
    ctx = c;
    parts = createParts();
    rig = new Rig();
    const { A } = buildScript(ctx, parts, rig);
   
    for (const [name, obj] of Object.entries(A)) if (obj && obj.isObject3D) ctx.labels.anchor(name, obj);
    // adapter-digest token of beat 7.14 sits in the entry row: anchor by position
    const s = ctx.camera.shots;
    const S = {
      IN: s.establish([-10, 1, 0], 30),
      DECK: s.inspect([-11, 0, -1], 9, 'top'),
      HASHC: s.inspect([-7.1, 0, -1.2], 13, 'top'),
      CAB: s.establish([3, 1.6, -2.6], 13),
      DRW: s.establish([3, 1.5, -2.2], 7),
      TL: s.inspect([0.5, 0.1, 3], 12, 'top'),
      TC: s.inspect([5, 0.1, 3], 13, 'top'),
      TR: s.inspect([10, 0.1, 3], 12, 'top'),
      BOTH: s.establish([4, 1, 0], 26),
      OUT: s.establish([10, 1, 0], 30),
    };
    const pose = ['IN', 'HASHC', 'CAB', 'DRW', 'CAB', 'CAB', 'TL', 'TL', 'TC', 'TC', 'BOTH', 'TR', 'TR', 'TR', 'TC', 'TC', 'TC', 'TC', 'TC', 'TR', 'BOTH'];
    const keys = [{ t: 0, pose: S.IN, dur: 2 }, { t: T0[0] + 9, pose: S.DECK, dur: 1.5 }];
    for (let i = 1; i < pose.length; i++) if (pose[i] !== pose[i - 1]) keys.push({ t: T0[i], pose: S[pose[i]], dur: 1.5 });
    keys.push({ t: T0[20] + 8, pose: S.OUT, dur: 1.5 });
    ctx.camera.track(keys);
  },

  seek(t) {
    rig.apply(t);
    for (const [name, content] of CARRIER) ctx.carrier.show(name, 1, content);
  },

  dispose() {
    if (parts) parts.dispose();
    ctx = null; parts = null; rig = null;
  },
};
