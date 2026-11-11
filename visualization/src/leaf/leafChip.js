// Leaf circuit view: the chip leaf of design 2.7 (proposed 2026-11-10), animated.
//
// The trace is the spike register file of crates/mori-chip/src/rf.rs, replayed in
// JavaScript: 32 register words, the six stand-in rules (Push, Rewrite, TPush,
// Arith, TPop, Pop) and genesis with len = 8. Sixteen steps give six appends and
// two padding rows, as in rf_leaf_s16_mock_prover_final_e8. Every size and row
// count shown comes from design.md 2.7.10-2.7.11 and experiments-4.md.

const SVG_NS = 'http://www.w3.org/2000/svg';
const S = 16;
const SEG_C = 8;

// Register words (rf.rs).
const R = {
  CTL: 0, CTL_A: 1, CTL_B: 2, CTL_C: 3, ORIG: 4, OUTC: 5, FUEL0: 6, FUEL: 7,
  WORK: 8, HCTR: 9, LIMIT: 10, FCNT: 11, TCNT: 12, ENV: 13, FP: 14, TP: 15,
  LEN: 16, TOP0: 17, TOP1: 18, TASK0: 19, TASK1: 20, H0: 21, H1: 22, H2: 23,
  H3: 24, H4: 25, H5: 26, R0: 27, R1: 28, V0: 29, V1: 30, V2: 31,
};
const REG_NAME = [
  'ctl', 'ctl_a', 'ctl_b', 'ctl_c', 'orig', 'outc', 'fuel0', 'fuel',
  'work', 'hctr', 'limit', 'fcount', 'tcount', 'env', 'fp', 'tp',
  'len', 'top0', 'top1', 'task0', 'task1', 'h0', 'h1', 'h2',
  'h3', 'h4', 'h5', 'r0', 'r1', 'v0', 'v1', 'v2',
];
const SCRATCH = new Set([R.V0, R.V1, R.V2]);

// Layout B slot (rfcore.rs, Arch::Narrow): 13 rows, pair rows between flag and lane rows.
const SLOT_ROWS = ['F', 0, 'L', 1, 'F', 2, 'L', 3, 'F', 4, 'L', 5, 'F'];
const PAIRS = [
  [R.CTL, R.FCNT, R.FP, R.TCNT, R.TP, R.LEN],
  [R.TOP0, R.TOP1, R.TASK0, R.TASK1, R.CTL_A, R.CTL_B],
  [R.H0, R.H1, R.R1, R.R0, R.V0, R.V1],
  [R.V2, R.CTL_C, R.ORIG, R.OUTC, R.FUEL0, R.FUEL],
  [R.WORK, R.HCTR, R.LIMIT, R.ENV, R.H2, R.H3],
  [R.H4, R.H5],
];

const RULES = ['Push', 'Pop', 'Rewrite', 'Arith', 'TPush', 'TPop'];
const T = { PUSH: 1, POP: 2, REWRITE: 3, ARITH: 4, TPUSH: 5, TPOP: 6, ERR: 7 };

function genesis() {
  const x = new Array(32).fill(0);
  x[R.CTL] = T.PUSH;
  x[R.CTL_A] = 1;
  x[R.CTL_B] = 1;
  x[R.CTL_C] = 5;
  x[R.FUEL0] = 1_000_000;
  x[R.FUEL] = 1_000_000;
  x[R.LIMIT] = 8;
  x[R.LEN] = SEG_C;
  return x;
}

function step(x, base, appended) {
  const y = x.slice();
  let rule;
  let alu = null;
  let mem = null;
  const lt = (a, b) => ({ op: 'lt', x: a, y: b, z: a < b ? 1 : 0 });
  switch (x[R.CTL]) {
    case T.PUSH: {
      alu = lt(x[R.FCNT], x[R.LIMIT]);
      if (alu.z !== 1) return null;
      const cell = [x[R.TOP0], x[R.TOP1], x[R.FP]];
      y[R.FP] = x[R.LEN];
      y[R.LEN] = x[R.LEN] + 1;
      y[R.TOP0] = x[R.CTL_A];
      y[R.TOP1] = x[R.CTL_B];
      y[R.FCNT] = x[R.FCNT] + 1;
      y[R.CTL] = T.REWRITE;
      rule = 0;
      mem = { kind: 'append', addr: x[R.LEN], cell };
      break;
    }
    case T.REWRITE:
      y[R.TOP0] = x[R.CTL_B];
      y[R.CTL] = T.TPUSH;
      rule = 2;
      break;
    case T.TPUSH: {
      alu = lt(x[R.TCNT], x[R.LIMIT]);
      if (alu.z !== 1) return null;
      const cell = [x[R.TASK0], x[R.TASK1], x[R.TP]];
      y[R.TP] = x[R.LEN];
      y[R.LEN] = x[R.LEN] + 1;
      y[R.TASK0] = x[R.CTL_A];
      y[R.TASK1] = x[R.CTL_C];
      y[R.TCNT] = x[R.TCNT] + 1;
      y[R.CTL] = T.ARITH;
      rule = 4;
      mem = { kind: 'append', addr: x[R.LEN], cell };
      break;
    }
    case T.ARITH: {
      const z = x[R.CTL_A] + x[R.CTL_B];
      alu = { op: 'add', x: x[R.CTL_A], y: x[R.CTL_B], z };
      y[R.R0] = z;
      y[R.CTL_A] = z;
      y[R.CTL] = T.TPOP;
      rule = 3;
      break;
    }
    case T.TPOP: {
      alu = lt(x[R.TP], x[R.LEN]);
      if (alu.z !== 1 || x[R.TCNT] === 0 || x[R.TP] < base) return null;
      const cell = appended[x[R.TP] - base];
      y[R.H0] = x[R.TASK0];
      y[R.H1] = x[R.TASK1];
      y[R.TASK0] = cell[0];
      y[R.TASK1] = cell[1];
      y[R.TP] = cell[2];
      y[R.TCNT] = x[R.TCNT] - 1;
      y[R.CTL] = T.POP;
      rule = 5;
      mem = { kind: 'load', addr: x[R.TP], cell };
      break;
    }
    case T.POP: {
      alu = lt(x[R.FP], x[R.LEN]);
      if (alu.z !== 1 || x[R.FCNT] === 0 || x[R.FP] < base) return null;
      const cell = appended[x[R.FP] - base];
      y[R.R1] = x[R.TOP0];
      y[R.TOP0] = cell[0];
      y[R.TOP1] = cell[1];
      y[R.FP] = cell[2];
      y[R.FCNT] = x[R.FCNT] - 1;
      y[R.CTL] = T.PUSH;
      rule = 1;
      mem = { kind: 'load', addr: x[R.FP], cell };
      break;
    }
    default:
      return null;
  }
  y[R.WORK] = x[R.WORK] + 1;
  y[R.FUEL] = x[R.FUEL] - 1;
  y[R.V0] = alu ? alu.z : 0;
  y[R.V1] = mem ? mem.cell[0] : 0;
  y[R.V2] = mem ? mem.cell[1] : 0;
  return { rule, tag: x[R.CTL], pre: x, post: y, alu, mem };
}

function runLeaf() {
  const pre = genesis();
  const base = pre[R.LEN];
  const steps = [];
  const appended = [];
  let x = pre;
  while (steps.length < S) {
    const s = step(x, base, appended);
    if (!s) break;
    if (s.mem && s.mem.kind === 'append') {
      appended.push(s.mem.cell);
      s.appendIndex = appended.length - 1;
    }
    steps.push(s);
    x = s.post;
  }
  return { pre, base, steps, appended, post: x };
}

// ---------------------------------------------------------------------------
// Layouts. Coordinates are in viewBox units; each layout is drawn as it is.

const LAYOUTS = {
  wide: {
    W: 1000, H: 478,
    ribbon: { x: 24, y: 12, w: 400, h: 20 },
    core: { x: 24, y: 44, w: 400, h: 420 },
    chips: {
      dec: { x: 500, y: 44, w: 262, h: 84 },
      page: { x: 500, y: 138, w: 262, h: 60 },
      alu: { x: 500, y: 208, w: 262, h: 74 },
      mem: { x: 500, y: 292, w: 262, h: 94 },
      hash: { x: 500, y: 396, w: 262, h: 68 },
    },
    bus: { kind: 'v', x: 462, from: 150, port: { x: 424, y: 244 } },
    decRoute: [[424, 86], [500, 86]],
    hashRoutes: [[[631, 386], [631, 396]], [[424, 430], [500, 430]]],
    fold: { x: 792, y: 44, w: 184, h: 420, dir: 'v' },
  },
  square: {
    W: 620, H: 640,
    ribbon: { x: 12, y: 10, w: 330, h: 20 },
    core: { x: 12, y: 40, w: 330, h: 420 },
    chips: {
      dec: { x: 380, y: 40, w: 228, h: 84 },
      page: { x: 380, y: 134, w: 228, h: 62 },
      alu: { x: 380, y: 206, w: 228, h: 74 },
      mem: { x: 380, y: 290, w: 228, h: 94 },
      hash: { x: 380, y: 394, w: 228, h: 66 },
    },
    bus: { kind: 'v', x: 361, from: 146, port: { x: 342, y: 240 } },
    decRoute: [[342, 82], [380, 82]],
    hashRoutes: [[[494, 384], [494, 394]], [[342, 427], [380, 427]]],
    fold: { x: 12, y: 478, w: 596, h: 152, dir: 'h' },
  },
  tall: {
    W: 380, H: 1086,
    ribbon: { x: 10, y: 8, w: 360, h: 20 },
    core: { x: 10, y: 36, w: 360, h: 410 },
    chips: {
      dec: { x: 10, y: 474, w: 166, h: 86 },
      page: { x: 204, y: 474, w: 166, h: 86 },
      alu: { x: 10, y: 572, w: 166, h: 76 },
      hash: { x: 204, y: 572, w: 166, h: 76 },
      mem: { x: 10, y: 660, w: 360, h: 96 },
    },
    bus: { kind: 'spine', x: 190, port: { x: 190, y: 446 } },
    decRoute: [[93, 446], [93, 474]],
    hashRoutes: [[[287, 660], [287, 648]]],
    fold: { x: 10, y: 772, w: 360, h: 304, dir: 'v' },
  },
};

function pickLayout(width) {
  if (width < 540) return 'tall';
  if (width >= 860) return 'wide';
  return 'square';
}

// ---------------------------------------------------------------------------
// Small SVG helpers.

function el(tag, attrs, parent) {
  const n = document.createElementNS(SVG_NS, tag);
  if (attrs) for (const k in attrs) n.setAttribute(k, attrs[k]);
  if (parent) parent.appendChild(n);
  return n;
}
function txt(parent, x, y, s, cls, extra) {
  const n = el('text', Object.assign({ x, y, class: cls }, extra || {}), parent);
  n.textContent = s;
  return n;
}
function setText(n, s) {
  if (n.__t !== s) {
    n.textContent = s;
    n.__t = s;
  }
}
function setCls(n, name, on) {
  if (n.classList.contains(name) !== on) n.classList.toggle(name, on);
}
function fmt(n) {
  // Cells hold four characters; the caption gives exact values.
  if (n >= 100000) return Math.round(n / 1e5) / 10 + 'M';
  if (n >= 10000) return Math.round(n / 1000) + 'k';
  return String(n);
}
function num(n) {
  return n.toLocaleString('en-US');
}
function polyLen(pts) {
  let L = 0;
  for (let i = 1; i < pts.length; i++) L += Math.hypot(pts[i][0] - pts[i - 1][0], pts[i][1] - pts[i - 1][1]);
  return L;
}
function pointAt(pts, f) {
  const L = polyLen(pts) * Math.max(0, Math.min(1, f));
  let acc = 0;
  for (let i = 1; i < pts.length; i++) {
    const seg = Math.hypot(pts[i][0] - pts[i - 1][0], pts[i][1] - pts[i - 1][1]);
    if (acc + seg >= L) {
      const u = seg ? (L - acc) / seg : 0;
      return [pts[i - 1][0] + (pts[i][0] - pts[i - 1][0]) * u, pts[i - 1][1] + (pts[i][1] - pts[i - 1][1]) * u];
    }
    acc += seg;
  }
  return pts[pts.length - 1];
}
const ease = (t) => 1 - Math.pow(1 - Math.max(0, Math.min(1, t)), 3);

// ---------------------------------------------------------------------------
// Phases.

function buildPhases(trace) {
  const out = [];
  trace.steps.forEach((s, k) => {
    const fast = k >= 6 ? 0.5 : 1;
    out.push({ kind: 'dec', k, ms: 620 * fast });
    if (s.alu) out.push({ kind: 'alu', k, ms: 700 * fast });
    if (s.mem) out.push({ kind: 'mem', k, ms: 820 * fast });
    out.push({ kind: 'write', k, ms: 760 * fast });
    out.push({ kind: 'copy', k, ms: 560 * fast });
  });
  out.push({ kind: 'close', k: S, ms: 3000 });
  out.push({ kind: 'records', k: S, ms: 3000 });
  out.push({ kind: 'walk', k: S, ms: 3000 });
  out.push({ kind: 'digest', k: S, ms: 3200 });
  out.push({ kind: 'fold', k: S, ms: 4200 });
  out.push({ kind: 'hold', k: S, ms: 1800 });
  return out;
}

function changedRegs(s) {
  const out = [];
  for (let j = 0; j < 32; j++) if (s.pre[j] !== s.post[j]) out.push(j);
  return out;
}

function captionFor(ph, trace) {
  const k = ph.k;
  const s = trace.steps[k];
  const slot = `Slot ${k + 1} of ${S}`;
  switch (ph.kind) {
    case 'dec':
      return [
        `${slot} · decode`,
        `The core reads its control tag, ${s.tag}, and looks it up in DEC, a fixed table in the key. The row it matches names the rule for this slot: ${RULES[s.rule]}. DEC has one row per class, so the match is unique.`,
      ];
    case 'alu':
      return s.alu.op === 'lt'
        ? [`${slot} · ALU`, `The core puts (ALU, lt, ${s.alu.x}, ${s.alu.y}, ${s.alu.z}) on the bus. A request is valid only if some ALU row holds the same tuple, and every active ALU row satisfies its relation. RANGE bounds each value with one limb lookup.`]
        : [`${slot} · ALU`, `The core puts (ALU, add, ${s.alu.x}, ${s.alu.y}, ${s.alu.z}) on the bus: a checked 128-bit add. The sum goes to r0 and ctl_a.`];
    case 'mem': {
      const m = s.mem;
      const tup = `(MEM, ${m.addr}, ${m.cell.join(', ')})`;
      if (m.kind === 'append') {
        return [
          `${slot} · MEM append`,
          `${RULES[s.rule]} appends the old ${s.rule === 0 ? 'top frame and its link fp' : 'top task and its link tp'} at len = ${m.addr}: the core puts ${tup} on the bus. Memory is write-once, so the address is fixed and len advances by exactly one. The stored link is the core's own ${s.rule === 0 ? 'fp' : 'tp'}, never a bus value.`,
        ];
      }
      return [
        `${slot} · MEM read`,
        `${RULES[s.rule]} reads the cell at ${s.rule === 1 ? 'fp' : 'tp'} = ${m.addr}, after the ALU checked ${m.addr} < len. The address comes from a register. The cell was appended in this leaf, so MEM serves it; a cell from an earlier leaf would come from PAGE.`,
      ];
    }
    case 'write': {
      const ch = changedRegs(s).map((j) => `${REG_NAME[j]} ${num(s.pre[j])}→${num(s.post[j])}`);
      return [
        `${slot} · post row`,
        `The post cells take the new values: ${ch.join(', ')}. A register that the rule does not name keeps its value; a gate over each pre/post pair checks that.`,
      ];
    }
    case 'copy':
      return [
        `${slot} · copy to slot ${Math.min(k + 2, S)}`,
        k + 1 < S
          ? `Copy constraints tie every post cell to the same register's pre cell in the next slot. The node's verifier reads only rotations −1, 0 and +1, so the 32 words sit in narrow pair rows linked by copies.`
          : `The last post row closes the leaf's register state.`,
      ];
    case 'close':
      return [
        'Leaf end · segment close',
        `After ${S} steps the leaf has appended ${trace.appended.length} cells at ${trace.base}–${trace.base + trace.appended.length - 1}. Rows ${trace.base + trace.appended.length}–${trace.base + SEG_C - 1} of the segment are padding and put the zero tuple on the bus; each cell shows its record tag, 1 for written and 0 for padding. post_d.len is the close cell base + 8 = ${trace.base + SEG_C}, so the next leaf starts on an aligned segment.`,
      ];
    case 'records':
      return [
        'Leaf end · segment root',
        'HASH turns each of the 8 rows into a record H(tag, a, b, c), with tag 1 on written rows and 0 on padding, and 7 compressions give the segment root. Its inputs are copies of the MEM cells, not bus requests.',
      ];
    case 'walk':
      return [
        'Leaf end · Merkle walk',
        `The old walk, depth 17, proves segment slot ${trace.base / SEG_C} empty in pre_d's mem_root. The new walk uses the same sibling and bit cells and gives post_d's mem_root.`,
      ];
    case 'digest':
      return [
        'Leaf end · state digest',
        'post_d is Poseidon over the 29 digested register words and mem_root; v0–v2 are scratch and stay out. pre_d is built the same way from slot 1. The next leaf must start from this post_d. Per leaf: 87 permutations, 6,090 of the 6,511 rows.',
      ];
    case 'fold':
    case 'hold':
      return [
        'Recursion · node and decider',
        'A recursion node at k = 20 folds the leaf proof into the run: 823,466 rows against a gate of 950,000. A run decider at k = 20 seals the run: 929,166 rows against 1,000,000. The node grows with the leaf\'s columns and lookups, not its rows, so the leaf is narrow and long.',
      ];
    default:
      return ['', ''];
  }
}

// ---------------------------------------------------------------------------
// The view.

export function mountLeafChip(root) {
  const trace = runLeaf();
  const phases = buildPhases(trace);
  const stepStart = phases.findIndex((p) => p.kind === 'close');

  const svgHost = root.querySelector('[data-leaf-svg]');
  const capTitle = root.querySelector('[data-leaf-cap-title]');
  const capText = root.querySelector('[data-leaf-cap-text]');
  const btnPlay = root.querySelector('[data-leaf-play]');
  const btnStep = root.querySelector('[data-leaf-step]');
  const btnEnd = root.querySelector('[data-leaf-end]');
  const btnRestart = root.querySelector('[data-leaf-restart]');
  const captionBox = root.querySelector('[data-leaf-caption]');

  const reduce = typeof window.matchMedia === 'function' && window.matchMedia('(prefers-reduced-motion: reduce)').matches;

  const st = {
    p: 0,
    t: 0,
    playing: !reduce,
    visible: true,
    active: true,
    layoutKey: null,
    raf: 0,
    last: 0,
    g: null,
  };

  // -------------------------------------------------------------------------
  function build(layoutKey) {
    const L = LAYOUTS[layoutKey];
    svgHost.innerHTML = '';
    const svg = el('svg', {
      viewBox: `0 0 ${L.W} ${L.H}`,
      class: `leaf-svg leaf-${layoutKey}`,
      role: 'img',
      'aria-labelledby': 'leafSvgTitle',
    }, svgHost);
    el('title', { id: 'leafSvgTitle' }, svg).textContent =
      'One leaf circuit: a CPU core with a 32-word register file in pre/post pair rows, and the chips DEC, PAGE, MEM, ALU with RANGE and HASH reached through a lookup bus. The leaf runs 16 steps, closes its memory segment, hashes its state digest and is folded by a recursion node and a run decider.';
    const defs = el('defs', null, svg);
    const pat = el('pattern', { id: 'leafHatch', width: 5, height: 5, patternUnits: 'userSpaceOnUse', patternTransform: 'rotate(45)' }, defs);
    el('line', { x1: 0, y1: 0, x2: 0, y2: 5, class: 'leaf-hatch-line' }, pat);

    const g = { svg, L, regs: [], ribbon: [], mem: [], dec: [], fold: {} };

    // Ribbon: the 16 step slots of the leaf.
    const rb = L.ribbon;
    const lblW = 64;
    txt(svg, rb.x, rb.y + 14, 'step slots', 'leaf-micro');
    const sw = (rb.w - lblW) / S;
    for (let i = 0; i < S; i++) {
      g.ribbon.push(el('rect', { x: rb.x + lblW + i * sw + 1, y: rb.y + 4, width: sw - 2, height: rb.h - 8, rx: 2, class: 'leaf-slot' }, svg));
    }

    // Wires first, so boxes sit on top.
    const wires = el('g', { class: 'leaf-wires' }, svg);
    const C = L.chips;
    const bus = L.bus;
    g.routes = {};
    if (bus.kind === 'v') {
      const bottom = C.mem.y + C.mem.h / 2;
      el('line', { x1: bus.x, y1: bus.from, x2: bus.x, y2: bottom, class: 'leaf-bus' }, wires);
      el('line', { x1: bus.port.x, y1: bus.port.y, x2: bus.x, y2: bus.port.y, class: 'leaf-bus' }, wires);
      for (const name of ['page', 'alu', 'mem']) {
        const c = C[name];
        const cy = c.y + c.h / 2;
        el('line', { x1: bus.x, y1: cy, x2: c.x, y2: cy, class: name === 'page' ? 'leaf-bus leaf-bus-ghost' : 'leaf-bus' }, wires);
        g.routes[name] = [[bus.port.x, bus.port.y], [bus.x, bus.port.y], [bus.x, cy], [c.x, cy]];
      }
      const lab = txt(wires, bus.x - 6, bus.from - 8, 'bus', 'leaf-micro leaf-bus-label', { 'text-anchor': 'middle' });
      lab.setAttribute('x', bus.x);
    } else {
      const end = C.mem.y;
      el('line', { x1: bus.x, y1: bus.port.y, x2: bus.x, y2: end, class: 'leaf-bus' }, wires);
      for (const name of ['page', 'alu']) {
        const c = C[name];
        const cy = c.y + c.h / 2;
        const edge = c.x > bus.x ? c.x : c.x + c.w;
        el('line', { x1: bus.x, y1: cy, x2: edge, y2: cy, class: name === 'page' ? 'leaf-bus leaf-bus-ghost' : 'leaf-bus' }, wires);
        g.routes[name] = [[bus.port.x, bus.port.y], [bus.x, cy], [edge, cy]];
      }
      g.routes.mem = [[bus.port.x, bus.port.y], [bus.x, end]];
      txt(wires, bus.x + 6, bus.port.y + 14, 'bus', 'leaf-micro leaf-bus-label');
    }
    g.routes.dec = L.decRoute;
    el('polyline', { points: L.decRoute.map((p) => p.join(',')).join(' '), class: 'leaf-lookup' }, wires);
    g.hashWires = L.hashRoutes.map((r) => el('polyline', { points: r.map((p) => p.join(',')).join(' '), class: 'leaf-copywire' }, wires));

    // CORE.
    const co = L.core;
    g.core = el('rect', { x: co.x, y: co.y, width: co.w, height: co.h, rx: 12, class: 'leaf-box leaf-core' }, svg);
    txt(svg, co.x + 14, co.y + 22, 'CORE', 'leaf-title');
    g.coreSlot = txt(svg, co.x + co.w - 14, co.y + 22, '', 'leaf-sub leaf-mono', { 'text-anchor': 'end' });
    txt(svg, co.x + 66, co.y + 22, 'register file, one slot', 'leaf-sub');

    const rx = co.x + 12;
    const rw = co.w - 24;
    const thin = 7;
    const pairH = 33;
    let y = co.y + 34;
    const cw = rw / 6;
    for (const row of SLOT_ROWS) {
      if (row === 'F' || row === 'L') {
        el('rect', { x: rx, y, width: rw, height: thin - 2, rx: 1.5, class: row === 'F' ? 'leaf-frow' : 'leaf-lrow' }, svg);
        y += thin;
        continue;
      }
      PAIRS[row].forEach((j, m) => {
        const x = rx + m * cw;
        const scratch = SCRATCH.has(j);
        txt(svg, x + cw / 2, y + 9, REG_NAME[j], scratch ? 'leaf-reg-name leaf-reg-scratch' : 'leaf-reg-name', { 'text-anchor': 'middle' });
        const hw = cw / 2 - 2;
        const preR = el('rect', { x: x + 1, y: y + 12, width: hw, height: pairH - 15, rx: 3, class: 'leaf-cell leaf-pre' + (scratch ? ' is-scratch' : '') }, svg);
        const postR = el('rect', { x: x + 1 + hw + 2, y: y + 12, width: hw, height: pairH - 15, rx: 3, class: 'leaf-cell leaf-post' + (scratch ? ' is-scratch' : '') }, svg);
        const preT = txt(svg, x + 1 + hw / 2, y + 12 + (pairH - 15) / 2 + 3.4, '', 'leaf-val', { 'text-anchor': 'middle' });
        const postT = txt(svg, x + 3 + hw * 1.5, y + 12 + (pairH - 15) / 2 + 3.4, '', 'leaf-val', { 'text-anchor': 'middle' });
        el('title', null, preR).textContent = `${REG_NAME[j]}, pre cell`;
        el('title', null, postR).textContent = `${REG_NAME[j]}, post cell`;
        g.regs[j] = { preR, postR, preT, postT };
      });
      y += pairH;
    }
    // Legend under the register file.
    y += 6;
    const lg = el('g', { class: 'leaf-legend' }, svg);
    el('rect', { x: rx, y: y + 1, width: 10, height: 8, rx: 2, class: 'leaf-cell leaf-pre' }, lg);
    txt(lg, rx + 14, y + 9, 'pre', 'leaf-micro');
    el('rect', { x: rx + 40, y: y + 1, width: 10, height: 8, rx: 2, class: 'leaf-cell leaf-post' }, lg);
    txt(lg, rx + 54, y + 9, 'post', 'leaf-micro');
    el('rect', { x: rx + 84, y: y + 2, width: 14, height: 5, class: 'leaf-frow' }, lg);
    txt(lg, rx + 102, y + 9, 'flags', 'leaf-micro');
    el('rect', { x: rx + 136, y: y + 2, width: 14, height: 5, class: 'leaf-lrow' }, lg);
    txt(lg, rx + 154, y + 9, 'bus lanes', 'leaf-micro');
    if (rw > 300) txt(lg, rx + rw, y + 9, 'v0–v2 scratch', 'leaf-micro leaf-reg-scratch', { 'text-anchor': 'end' });

    // TRAP band (not built in the spike).
    y += 18;
    el('rect', { x: rx, y, width: rw, height: 24, rx: 6, class: 'leaf-ghost' }, svg);
    txt(svg, rx + 10, y + 16, 'TRAP', 'leaf-chipname leaf-ghost-text');
    txt(svg, rx + 50, y + 16, rw > 320 ? 'P0–P2 gates in the core · not built' : 'gates in core · not built', 'leaf-micro leaf-ghost-text');

    // Digest pills.
    y += 32;
    const pillW = (rw - 10) / 2;
    g.preD = el('rect', { x: rx, y, width: pillW, height: 24, rx: 12, class: 'leaf-digest' }, svg);
    txt(svg, rx + pillW / 2, y + 16, 'pre_d', 'leaf-chipname', { 'text-anchor': 'middle' });
    g.postD = el('rect', { x: rx + pillW + 10, y, width: pillW, height: 24, rx: 12, class: 'leaf-digest' }, svg);
    g.postDText = txt(svg, rx + pillW + 10 + pillW / 2, y + 16, 'post_d', 'leaf-chipname', { 'text-anchor': 'middle' });
    g.postDCenter = [rx + pillW + 10 + pillW / 2, y + 12];

    // Chips.
    const chipBox = (name, title, sub, ghost) => {
      const c = C[name];
      const r = el('rect', { x: c.x, y: c.y, width: c.w, height: c.h, rx: 10, class: 'leaf-box leaf-chip' + (ghost ? ' leaf-ghost' : '') }, svg);
      txt(svg, c.x + 12, c.y + 19, title, 'leaf-chipname' + (ghost ? ' leaf-ghost-text' : ''));
      const subs = Array.isArray(sub) ? sub : [sub];
      subs.forEach((s, i) => txt(svg, c.x + 12, c.y + 34 + i * 12, s, 'leaf-micro' + (ghost ? ' leaf-ghost-text' : '')));
      return r;
    };
    const narrow = C.dec.w < 200;
    g.chip = {};
    g.chip.dec = chipBox('dec', 'DEC', narrow ? 'control tag → rule' : 'decoder: control tag → rule, fixed table');
    {
      const c = C.dec;
      // One row per rule; six across when they fit, else two rows of three.
      const cols = c.w - 24 >= 6 * 44 ? 6 : 3;
      const rowsN = 6 / cols;
      const cw2 = (c.w - 24) / cols;
      const rh = rowsN === 1 ? 22 : 15;
      const top = c.y + c.h - 8 - rowsN * (rh + 2);
      RULES.forEach((name, i) => {
        const x = c.x + 12 + (i % cols) * cw2;
        const yy = top + Math.floor(i / cols) * (rh + 2);
        const r = el('rect', { x: x + 1, y: yy, width: cw2 - 2, height: rh, rx: 3, class: 'leaf-cell leaf-decrow' }, svg);
        txt(svg, x + cw2 / 2, yy + rh / 2 + 3.3, name, 'leaf-val', { 'text-anchor': 'middle' });
        g.dec.push(r);
      });
    }
    g.chip.page = chipBox('page', 'PAGE', narrow ? ['older rows and cells', 'not built'] : ['older PROG rows and cells, opened', 'against prog_root and mem_root · not built'], true);
    g.chip.alu = chipBox('alu', 'ALU + RANGE', narrow ? ['128-bit ops', 'RANGE: one limb lookup'] : ['128-bit add, sub, mul, compare', 'RANGE: one limb lookup per value']);
    g.aluOp = txt(svg, C.alu.x + 12, C.alu.y + C.alu.h - 11, '', 'leaf-val leaf-mono-lg');
    g.chip.mem = chipBox('mem', 'MEM', narrow ? 'write-once segment' : 'write-once segment, 8 cells per leaf');
    g.memLen = txt(svg, C.mem.x + C.mem.w - 12, C.mem.y + 19, '', 'leaf-micro leaf-mono', { 'text-anchor': 'end' });
    {
      const c = C.mem;
      const mw = (c.w - 24) / SEG_C;
      for (let i = 0; i < SEG_C; i++) {
        const x = c.x + 12 + i * mw;
        const r = el('rect', { x: x + 1.5, y: c.y + c.h - 46, width: mw - 3, height: 26, rx: 3, class: 'leaf-cell leaf-memcell' }, svg);
        txt(svg, x + mw / 2, c.y + c.h - 8, String(trace.base + i), 'leaf-micro', { 'text-anchor': 'middle' });
        const t = txt(svg, x + mw / 2, c.y + c.h - 29.5, '', 'leaf-val', { 'text-anchor': 'middle' });
        el('title', null, r).textContent = `address ${trace.base + i}`;
        g.mem.push({ r, t });
      }
    }
    g.chip.hash = chipBox('hash', 'HASH', narrow ? 'Poseidon, fed by copies' : 'Poseidon only; inputs are copied cells');
    {
      const c = C.hash;
      g.hashBarBg = el('rect', { x: c.x + 12, y: c.y + c.h - 20, width: c.w - 24, height: 8, rx: 4, class: 'leaf-bar-bg' }, svg);
      g.hashBar = el('rect', { x: c.x + 12, y: c.y + c.h - 20, width: 0, height: 8, rx: 4, class: 'leaf-bar' }, svg);
      g.hashCount = txt(svg, c.x + c.w - 12, c.y + 19, '', 'leaf-micro leaf-mono', { 'text-anchor': 'end' });
      g.hashW = c.w - 24;
    }

    // Fold: leaf proof, recursion node, run decider.
    const F = L.fold;
    const items = [
      { key: 'leaf', title: 'Leaf proof', rows: 6511, k: 13, gate: null, note: `S = ${S} steps; 6,090 rows are Poseidon` },
      { key: 'node', title: 'Recursion node', rows: 823466, k: 20, gate: 950000, note: 'folds each leaf proof; headroom 126,534' },
      { key: 'dec', title: 'Run decider', rows: 929166, k: 20, gate: 1000000, note: 'seals the run; headroom 70,834' },
    ];
    const gap = 12;
    const n = items.length;
    const bw = F.dir === 'h' ? (F.w - gap * (n - 1)) / n : F.w;
    const bh = F.dir === 'h' ? F.h : (F.h - gap * (n - 1)) / n;
    const scaleMax = 1000000;
    g.foldBoxes = [];
    items.forEach((it, i) => {
      const x = F.dir === 'h' ? F.x + i * (bw + gap) : F.x;
      const yy = F.dir === 'h' ? F.y : F.y + i * (bh + gap);
      const box = el('rect', { x, y: yy, width: bw, height: bh, rx: 10, class: 'leaf-box leaf-fold' }, svg);
      // Title row, note, number row, bar. The bar scale is 1,000,000 rows in every box.
      txt(svg, x + 12, yy + 20, it.title, 'leaf-chipname');
      const notes = bw < 220 && it.note.includes('; ') ? it.note.split('; ') : [it.note];
      notes.forEach((nt, ni) => txt(svg, x + 12, yy + 35 + ni * 12, nt, 'leaf-micro'));
      const numY = yy + bh - 30;
      const big = txt(svg, x + 12, numY, `${num(it.rows)}`, 'leaf-num');
      txt(svg, x + 12 + num(it.rows).length * 11.6 + 8, numY, `rows · k = ${it.k}`, 'leaf-micro leaf-mono');
      const by = yy + bh - 18;
      el('rect', { x: x + 12, y: by, width: bw - 24, height: 7, rx: 3.5, class: 'leaf-bar-bg' }, svg);
      const fill = el('rect', { x: x + 12, y: by, width: 0, height: 7, rx: 3.5, class: 'leaf-bar' }, svg);
      const label = it.gate ? `gate ${num(it.gate)} rows` : 'bar scale 1,000,000 rows';
      if (it.gate) {
        const gx = x + 12 + (bw - 24) * (it.gate / scaleMax);
        el('line', { x1: gx, y1: by - 3, x2: gx, y2: by + 10, class: 'leaf-gate' }, svg);
      }
      if (bw < 220) txt(svg, x + 12, yy + 35 + notes.length * 12 + 2, label, 'leaf-micro' + (it.gate ? ' leaf-gate-label' : ''));
      else txt(svg, x + bw - 12, yy + 20, label, 'leaf-micro' + (it.gate ? ' leaf-gate-label' : ''), { 'text-anchor': 'end' });
      g.foldBoxes.push({ box, fill, frac: it.rows / scaleMax, w: bw - 24, big, cx: x + bw / 2, cy: yy + bh / 2 });
    });
    // Route of the leaf proof from post_d to the leaf box, around the chips.
    {
      const pd = g.postDCenter;
      const lb = g.foldBoxes[0];
      const coreBottom = co.y + co.h;
      if (layoutKey === 'wide') {
        const y = coreBottom + 7;
        g.proofRoute = [pd, [pd[0], y], [F.x - 14, y], [F.x - 14, lb.cy], [lb.cx, lb.cy]];
      } else if (layoutKey === 'square') {
        const y = (coreBottom + F.y) / 2;
        g.proofRoute = [pd, [pd[0], y], [lb.cx, y], [lb.cx, lb.cy]];
      } else {
        g.proofRoute = [pd, [L.bus.x, pd[1]], [L.bus.x, lb.cy]];
      }
    }
    // Fold arrows between boxes.
    for (let i = 0; i < n - 1; i++) {
      if (F.dir === 'h') {
        const x1 = F.x + (i + 1) * (bw + gap) - gap;
        el('line', { x1, y1: F.y + 20, x2: x1 + gap, y2: F.y + 20, class: 'leaf-foldlink' }, svg);
      } else {
        const y1 = F.y + (i + 1) * (bh + gap) - gap;
        el('line', { x1: F.x + 30, y1, x2: F.x + 30, y2: y1 + gap, class: 'leaf-foldlink' }, svg);
      }
    }

    // Packet: a bus tuple in flight.
    const pk = el('g', { class: 'leaf-packet', opacity: 0 }, svg);
    const pkR = el('rect', { x: 0, y: -10, width: 10, height: 20, rx: 10 }, pk);
    const pkT = txt(pk, 0, 4, '', 'leaf-packet-text');
    g.packet = { g: pk, r: pkR, t: pkT };

    st.g = g;
    st.layoutKey = layoutKey;
  }

  // -------------------------------------------------------------------------
  function packetShow(route, f, label) {
    const { g: pg, r, t } = st.g.packet;
    setText(t, label);
    const w = Math.max(40, label.length * 6.1 + 18);
    r.setAttribute('width', w);
    r.setAttribute('x', -w / 2);
    t.setAttribute('x', 0);
    t.setAttribute('text-anchor', 'middle');
    const [x, y] = pointAt(route, f);
    pg.setAttribute('transform', `translate(${x.toFixed(1)},${y.toFixed(1)})`);
    pg.setAttribute('opacity', 1);
  }
  function packetHide() {
    st.g.packet.g.setAttribute('opacity', 0);
  }

  function render() {
    const g = st.g;
    if (!g) return;
    const ph = phases[st.p];
    const t = reduce ? 1 : st.t;
    const k = ph.k;
    const s = k < S ? trace.steps[k] : null;
    const boundary = k >= S;

    // Ribbon and slot label.
    g.ribbon.forEach((r, i) => {
      setCls(r, 'is-done', i < k);
      setCls(r, 'is-on', i === k);
    });
    setText(g.coreSlot, boundary ? `after slot ${S}` : `slot ${k + 1} / ${S}`);

    // Registers.
    const pre = boundary ? trace.post : s.pre;
    const showPost = !boundary && (ph.kind === 'write' || ph.kind === 'copy');
    const changed = s ? new Set(changedRegs(s)) : new Set();
    for (let j = 0; j < 32; j++) {
      const c = g.regs[j];
      const copyDone = ph.kind === 'copy' && t > 0.55;
      const preVal = copyDone ? s.post[j] : pre[j];
      setText(c.preT, fmt(preVal));
      setText(c.postT, showPost ? fmt(s.post[j]) : '');
      setCls(c.postR, 'is-written', showPost && changed.has(j));
      setCls(c.preR, 'is-copied', copyDone && changed.has(j));
      setCls(c.postR, 'is-linking', ph.kind === 'copy' && !copyDone);
    }

    // DEC rows.
    g.dec.forEach((r, i) => setCls(r, 'is-on', !!s && i === s.rule && ph.kind !== 'copy'));

    // Chips glow.
    const glow = { dec: ph.kind === 'dec', alu: ph.kind === 'alu', mem: ph.kind === 'mem' || ph.kind === 'close', hash: ['records', 'walk', 'digest'].includes(ph.kind), page: false };
    for (const name in g.chip) setCls(g.chip[name], 'is-active', !!glow[name]);
    setCls(g.core, 'is-active', !boundary && ph.kind !== 'copy');

    // ALU readout.
    let lastAlu = null;
    for (let i = Math.min(k, S - 1); i >= 0; i--) {
      if (i === k && !['alu', 'mem', 'write', 'copy'].includes(ph.kind)) continue;
      if (trace.steps[i] && trace.steps[i].alu) {
        lastAlu = trace.steps[i].alu;
        break;
      }
    }
    setText(g.aluOp, lastAlu ? `${lastAlu.op}(${lastAlu.x}, ${lastAlu.y}) = ${lastAlu.z}` : '');

    // MEM cells.
    let lenNow = trace.base;
    trace.steps.forEach((st2, i) => {
      if (st2.mem && st2.mem.kind === 'append') {
        const done = i < k || (i === k && (['write', 'copy'].includes(ph.kind) || (ph.kind === 'mem' && t > 0.62)));
        if (done) lenNow += 1;
      }
    });
    if (boundary) lenNow = trace.base + trace.appended.length;
    g.mem.forEach((m, i) => {
      const written = trace.base + i < lenNow;
      const padding = boundary && !written;
      const loadHit = s && ph.kind === 'mem' && s.mem.kind === 'load' && s.mem.addr === trace.base + i && t > 0.5;
      const appendHit = s && ph.kind === 'mem' && s.mem.kind === 'append' && s.mem.addr === trace.base + i && t > 0.62;
      setCls(m.r, 'is-written', written);
      setCls(m.r, 'is-padding', padding);
      setCls(m.r, 'is-hit', loadHit || appendHit);
      setText(m.t, written ? '1' : padding ? '0' : '');
    });
    const closeShown = boundary && ph.kind !== 'close' ? true : ph.kind === 'close' && t > 0.5;
    setText(g.memLen, closeShown ? `len ${trace.base + SEG_C} (close)` : `len ${lenNow}`);

    // HASH counter: 16 record permutations + 7 compressions, 34 walk, 30 digest.
    let perms = 0;
    if (ph.kind === 'records') perms = 23 * ease(t);
    else if (ph.kind === 'walk') perms = 23 + 34 * ease(t);
    else if (ph.kind === 'digest') perms = 57 + 30 * ease(t);
    else if (ph.kind === 'fold' || ph.kind === 'hold') perms = 87;
    setText(g.hashCount, boundary && ph.kind !== 'close' ? `${Math.round(perms)} / 87 permutations` : '87 per leaf');
    g.hashBar.setAttribute('width', ((perms / 87) * g.hashW).toFixed(1));
    g.hashWires.forEach((w) => setCls(w, 'is-active', ['records', 'walk', 'digest'].includes(ph.kind)));

    // Digests.
    const digestOn = ph.kind === 'digest' ? t > 0.4 : ph.kind === 'fold' || ph.kind === 'hold';
    setCls(g.preD, 'is-on', digestOn);
    setCls(g.postD, 'is-on', digestOn);

    // Fold boxes.
    let foldF = 0;
    if (ph.kind === 'fold') foldF = t;
    else if (ph.kind === 'hold') foldF = 1;
    const seg = (a, b) => Math.max(0, Math.min(1, (foldF - a) / (b - a)));
    const fills = [seg(0, 0.3), seg(0.3, 0.65), seg(0.65, 1)];
    g.foldBoxes.forEach((b, i) => {
      b.fill.setAttribute('width', (Math.max(fills[i] > 0 ? 3 : 0, b.frac * b.w * ease(fills[i]))).toFixed(1));
      setCls(b.box, 'is-active', fills[i] > 0 && (fills[i] < 1 || ph.kind === 'hold'));
    });

    // Packet in flight.
    if (ph.kind === 'dec' && s) {
      const f = t < 0.5 ? ease(t / 0.5) : 1 - ease((t - 0.5) / 0.5);
      packetShow(g.routes.dec, f, t < 0.5 ? `tag ${s.tag}` : RULES[s.rule]);
    } else if (ph.kind === 'alu' && s) {
      const f = t < 0.55 ? ease(t / 0.55) : 1 - ease((t - 0.55) / 0.45);
      packetShow(g.routes.alu, f, `(ALU, ${s.alu.op}, ${s.alu.x}, ${s.alu.y}, ${s.alu.z})`);
    } else if (ph.kind === 'mem' && s) {
      const m = s.mem;
      const label = `(MEM, ${m.addr}, ${m.cell.join(', ')})`;
      const f = m.kind === 'append' ? ease(t / 0.62) : t < 0.5 ? ease(t / 0.5) : 1 - ease((t - 0.5) / 0.5);
      if (m.kind === 'append' && t > 0.7) packetHide();
      else packetShow(g.routes.mem, f, label);
    } else if (ph.kind === 'fold') {
      const b = [g.foldBoxes[0].cx, g.foldBoxes[0].cy];
      const c2 = [g.foldBoxes[1].cx, g.foldBoxes[1].cy];
      const d = [g.foldBoxes[2].cx, g.foldBoxes[2].cy];
      if (t < 0.3) packetShow(g.proofRoute, ease(t / 0.3), 'leaf proof');
      else if (t < 0.65) packetShow([b, c2], ease((t - 0.3) / 0.35), 'leaf proof');
      else if (t < 0.95) packetShow([c2, d], ease((t - 0.65) / 0.3), 'node proof');
      else packetHide();
    } else {
      packetHide();
    }

    // Caption.
    const [ct, cx] = captionFor(ph, trace);
    setText(capTitle, ct);
    setText(capText, cx);
  }

  // -------------------------------------------------------------------------
  function relayout() {
    const w = svgHost.clientWidth;
    if (!w) return; // hidden: keep the current drawing
    const key = pickLayout(w);
    if (key !== st.layoutKey) {
      build(key);
      render();
    }
  }

  function setPlaying(on) {
    st.playing = on;
    btnPlay.textContent = on ? 'Pause' : 'Play';
    btnPlay.setAttribute('aria-pressed', on ? 'true' : 'false');
    captionBox.setAttribute('aria-live', on ? 'off' : 'polite');
    ensureLoop();
  }

  function go(p, t = 0) {
    st.p = ((p % phases.length) + phases.length) % phases.length;
    st.t = t;
    render();
  }

  function frame(now) {
    st.raf = 0;
    if (!st.playing || !st.visible || !st.active || document.hidden) return;
    const dt = st.last ? Math.min(100, now - st.last) : 16;
    st.last = now;
    const ph = phases[st.p];
    st.t += dt / ph.ms;
    if (st.t >= 1) {
      if (st.p === phases.length - 1) go(0, 0);
      else go(st.p + 1, 0);
    } else {
      render();
    }
    st.raf = requestAnimationFrame(frame);
  }
  function ensureLoop() {
    if (st.raf || !st.playing || !st.visible || !st.active || document.hidden) return;
    st.last = 0;
    st.raf = requestAnimationFrame(frame);
  }

  btnPlay.addEventListener('click', () => setPlaying(!st.playing));
  btnStep.addEventListener('click', () => {
    setPlaying(false);
    go(st.p + 1, reduce ? 1 : 0.999);
  });
  btnEnd.addEventListener('click', () => {
    go(stepStart, reduce ? 1 : 0);
  });
  btnRestart.addEventListener('click', () => {
    go(0, reduce ? 1 : 0);
  });

  if (typeof ResizeObserver === 'function') {
    new ResizeObserver(() => relayout()).observe(svgHost);
  } else {
    window.addEventListener('resize', relayout);
  }
  if (typeof IntersectionObserver === 'function') {
    new IntersectionObserver((entries) => {
      st.visible = entries.some((e) => e.isIntersecting);
      ensureLoop();
    }).observe(root);
  }
  document.addEventListener('visibilitychange', ensureLoop);

  relayout();
  if (!st.g) build(pickLayout(600));
  go(0, reduce ? 1 : 0);
  setPlaying(st.playing);

  return {
    setActive(on) {
      st.active = on;
      if (on) relayout();
      ensureLoop();
    },
    state: st,
    trace,
    phases,
  };
}

// ---------------------------------------------------------------------------
// View switch between the leaf circuit and the reference machine (3D chamber).

export function initStageViews() {
  const card = document.querySelector('.stage-viewport-card');
  const leafRoot = document.getElementById('leafChipView');
  const column = document.querySelector('.stage-column');
  if (!card || !leafRoot) return null;
  const tabs = Array.from(document.querySelectorAll('[data-stage-view]'));
  const leaf = mountLeafChip(leafRoot);

  function setView(view) {
    const isLeaf = view === 'leaf';
    document.documentElement.dataset.stageView = view;
    card.classList.toggle('is-leaf', isLeaf);
    if (column) column.classList.toggle('is-leaf-view', isLeaf);
    leafRoot.hidden = !isLeaf;
    tabs.forEach((b) => {
      const on = b.dataset.stageView === view;
      b.setAttribute('aria-selected', on ? 'true' : 'false');
      b.tabIndex = on ? 0 : -1;
    });
    leaf.setActive(isLeaf);
    if (!isLeaf) window.dispatchEvent(new Event('resize'));
  }

  tabs.forEach((b, i) => {
    b.addEventListener('click', () => setView(b.dataset.stageView));
    b.addEventListener('keydown', (ev) => {
      if (ev.key !== 'ArrowRight' && ev.key !== 'ArrowLeft') return;
      ev.preventDefault();
      const next = tabs[(i + (ev.key === 'ArrowRight' ? 1 : tabs.length - 1)) % tabs.length];
      next.focus();
      setView(next.dataset.stageView);
    });
  });

  // Controls that drive the 3D machine bring its view back.
  const toMachine = () => setView('machine');
  for (const id of ['cameraLensBar', 'presetStrip', 'stagePresetStrip']) {
    const n = document.getElementById(id);
    if (n) n.addEventListener('click', (ev) => { if (ev.target.closest('button')) toMachine(); });
  }
  const run = document.getElementById('compileRunBtn');
  if (run) run.addEventListener('click', toMachine);

  setView('leaf');
  return { setView, leaf };
}
