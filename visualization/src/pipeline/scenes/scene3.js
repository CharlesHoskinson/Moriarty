import { BEATS } from './scene3/beats.js';
import { LP } from './scene3/labelpos.js';
import * as D from './scene3/draw.js';
import { setTime, START } from './scene3/time.js';
import { drawTape, drawBrackets } from './scene3/tape.js';
import { drawCore, drawDec, drawRange, drawAlu } from './scene3/core.js';
import { drawBus } from './scene3/bus.js';
import { drawHash, drawVd, drawExtras } from './scene3/hashvd.js';
import { drawStacks, drawInit, drawGenesis, drawBoundary } from './scene3/stacks.js';
import { buildSlabs, disposeSlabs, drawDock, drawRules, drawInset, drawCost } from './scene3/final.js';
import { drawMem, drawBackend, drawTree, drawPage, drawBudget } from './scene3/mem.js';
import { planOn } from './scene3/time.js';

// Scene 3: the leaf. One floor plan that the camera visits region by region (STORYBOARD scene 3).
let ctx = null;

const beats = BEATS.map((b, i) => ({
  id: b.id,
  title: b.title,
  duration: b.duration,
  narration: b.narration,
  labels: b.labels.map((l, k) => ({ l, p: LP[i][k] })).filter(({ p }) => !p.skip).map(({ l, p }, k) => (
    { id: `l${k}`, text: l.text, kind: p.kind || l.kind || 'neutral', at: [p.at[0], p.at[1], p.at[2]], dy: p.dy ?? 0.8, from: p.from, to: p.to })),
}));

function cameraKeys(s) {
  const IN = s.establish([-10, 1, -2], 30);
  const PLAN = s.inspect([0, 0, 0.3], 30, 'top');
  const CORE = s.inspect([-8, 0, 0], 10, 'top');
  const DEC = s.inspect([-3.5, 0, -2.6], 15, 'top');
  const BUSV = s.inspect([-2, 0, 0.7], 13, 'top');
  const ALU = s.inspect([0.5, 0, -1.7], 12, 'top');
  const RANGE = s.inspect([3, 0, -4.0], 9, 'top');
  const HASH = s.inspect([1, 0, 2.1], 9, 'top');
  const CHAIN = s.inspect([-4.5, 0, 1.2], 14, 'top');
  const MEMV = s.inspect([5.5, 0, 0.5], 17, 'top');
  const COST = s.establish([2, 1, 2.5], 30);
  const TREE = s.establish([9.5, 2.8, -1.5], 13);
  const READ = s.inspect([2.5, 0, 0.5], 19, 'top');
  const PAGE = s.inspect([4, 0, 2.2], 17, 'top');
  const STACK = s.establish([-6.3, 1.6, 4.3], 8);
  const VD = s.inspect([3, 0, 2.2], 11, 'top');
  const BND = s.inspect([9.5, 0, 3.6], 12, 'top');
  const INIT = s.inspect([-8.5, 0, 2.6], 12, 'top');
  const RIB = s.inspect([0.5, 0, -4.9], 19, 'top');
  const RIBW = s.inspect([4, 0, -4.2], 25, 'top');
  const DOCK = s.establish([13.2, 1, -3.4], 10);
  const INSET = s.inspect([3, 1.6, 0.6], 14, 'top');
  const OUT = s.establish([10, 1, 0], 30);
  const f = (x, z) => s.follow([x, 0.3, z], [-2, 9, 7]);
  const poses = [IN, IN, DEC, RIB, BUSV, null, ALU, ALU, RANGE, HASH, CHAIN, MEMV, MEMV, TREE, TREE, MEMV, READ, TREE, PAGE, PAGE,
    STACK, STACK, STACK, VD, INSET, BND, BND, BND, INIT, INIT, RIBW, DOCK, INSET, COST, COST];
  const keys = [];
  let prev = null;
  poses.forEach((p, i) => {
    if (i === 5) {
      keys.push({ t: START[5], pose: f(-3.3, -1.7), dur: 1.5 });
      keys.push({ t: START[5] + 2.0, pose: f(-0.6, -1.7), dur: 1.4 });
      keys.push({ t: START[5] + 6.0, pose: f(-4.7, -0.6), dur: 1.6 });
      prev = null;
      return;
    }
    if (p !== prev) keys.push({ t: START[i], pose: p, dur: keys.length ? 1.5 : 2 });
    prev = p;
  });
  keys.push({ t: START[1] + 4.0, pose: PLAN, dur: 1.5 });
  keys.push({ t: START[1] + 9.5, pose: CORE, dur: 1.5 });
  keys.push({ t: START[34] + 8.0, pose: OUT, dur: 1.5 });
  return keys.filter((k) => !(k.t === START[2] && false));
}

export default {
  id: 'leaf',
  title: 'The leaf',
  beats,

  build(c) {
    ctx = c;
    D.build(ctx.scene);
    buildSlabs(ctx.scene);
    ctx.camera.track(cameraKeys(ctx.camera.shots));
  },

  seek(t) {
    const b = ctx.beatAt(t);
    setTime(t, b);
    D.begin();
    drawTape();
    drawBrackets();
    if (planOn()) {
      drawCore();
      drawDec();
      drawRange();
      drawAlu();
      drawBus();
      drawMem();
      drawBackend();
      drawTree();
      drawPage();
      drawHash();
      drawVd();
      drawExtras();
      drawStacks();
      drawInit();
      drawGenesis();
      drawBoundary();
    }
    drawBudget();
    drawDock();
    drawRules();
    drawInset();
    drawCost();
    D.end();
  },

  dispose() {
    D.dispose();
    disposeSlabs();
    ctx = null;
  },
};
