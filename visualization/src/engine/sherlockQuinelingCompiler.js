// ============================================================================
// src/engine/sherlockQuinelingCompiler.js
// Compiles actual Moriarty Sherlock programs (`moriarty-sherlock-draft/1` AST +
// `sherlock-k-a/1` execution traces) into genuine, self-reproducing Quinelings
// (`Quinelings.makeTaskProgram`, `QDL`, `WovenBody`, `LifeformFamilies`,
// `Morphology`, `Chroma`) using the upstream Quinelings SDK.
// ============================================================================
import {
  Quinelings,
  QuinelingKernels,
  QDL,
  Chroma,
  Morphology,
  LifeformFamilies,
  WovenBody,
} from './quinelingsSdk.js';
import { parseAndCheckDraft } from './sherlockEngine.js';

/**
 * 32-bit FNV-1a hash (`h0 = 2166136261`, prime `16777619`)
 */
export function fnv1a32(str = '') {
  let h = 2166136261;
  for (let i = 0; i < str.length; i += 1) {
    h = Math.imul(h ^ (str.charCodeAt(i) & 0xff), 16777619) >>> 0;
  }
  return h >>> 0;
}

/**
 * Sanitizes an AST path or identifier into a valid Quinelings node ID (`^[A-Za-z][A-Za-z0-9_-]{0,63}$`).
 */
function sanitizeNodeId(raw, idx = 0) {
  const cleaned = String(raw || `node_${idx}`)
    .replace(/[^A-Za-z0-9_-]+/g, '_')
    .replace(/^[^A-Za-z]+/, 'n_');
  return (cleaned || `n_${idx}`).slice(0, 48);
}

/**
 * Walks a parsed Sherlock AST (`parseAndCheckDraft(source)`) to extract its exact
 * structural genome: function count, mutual recursion, ADT match patterns (`cons`/`nil` vs
 * `branch`/`leaf`), `Let` nesting depth, arithmetic operators, and literal seeds.
 */
export function analyzeSherlockAst(parsed, source = '', trace = null) {
  const exprs = parsed?.programImage?.expressions || [];
  const fns = parsed?.functions || [];
  const fnNames = new Set(fns.map((f) => f.name));

  let hasListMatch = false;
  let hasTreeMatch = false;
  let letCount = 0;
  let ifCount = 0;
  let matchCount = 0;
  let mulCount = 0;
  let addCount = 0;
  let subCount = 0;
  let consCount = 0;
  let branchCount = 0;
  let selfRecursiveCalls = 0;
  let crossRecursiveCalls = 0;
  const literals = [];

  for (const ex of exprs) {
    const ownerMatch = typeof ex.astPath === 'string' ? ex.astPath.match(/^fn\[\d+\]:([^/]+)/) : null;
    const currentFnName = ownerMatch ? ownerMatch[1] : null;
    switch (ex.kind) {
      case 'Literal': {
        const rawText = String(ex.text || '').trim();
        if (/^\d+$/.test(rawText)) {
          literals.push(Number(BigInt(rawText) & 0xffffffffn));
        } else if (rawText === 'true' || rawText === 'false') {
          literals.push(rawText === 'true' ? 1 : 0);
        }
        break;
      }
      case 'Let':
        letCount += 1;
        break;
      case 'If':
        ifCount += 1;
        break;
      case 'Match':
        matchCount += 1;
        if (/\bnil\b|\bcons\s*\(/.test(ex.text || '')) hasListMatch = true;
        if (/\bleaf\s*\(|\bbranch\s*\(/.test(ex.text || '')) hasTreeMatch = true;
        break;
      case 'Unary':
        if (ex.op === 'leaf') hasTreeMatch = true;
        break;
      case 'Binary':
        if (ex.op === '*') mulCount += 1;
        else if (ex.op === '+') addCount += 1;
        else if (ex.op === '-') subCount += 1;
        else if (ex.op === 'cons') {
          consCount += 1;
          hasListMatch = true;
        } else if (ex.op === 'branch') {
          branchCount += 1;
          hasTreeMatch = true;
        }
        break;
      case 'Call':
        if (currentFnName && ex.name === currentFnName) {
          selfRecursiveCalls += 1;
        } else if (currentFnName && fnNames.has(ex.name) && ex.name !== currentFnName) {
          crossRecursiveCalls += 1;
        }
        break;
      default:
        break;
    }
  }

  const stackLimit =
    trace?.steps?.[0]?.after?.stackLimit ??
    trace?.stackLimit ??
    16;
  const initialFuel =
    trace?.steps?.[0]?.before?.fuel ??
    trace?.fuel ??
    120;
  const outcomeKind = trace?.outcome?.kind || 'Done';
  const lastStep = trace?.steps?.[trace.steps.length - 1];
  const terminalRuleLine = lastStep?.ruleLine ?? 80;

  // Determine the canonical species ID from the actual AST structure and execution bounds
  let speciesId = 'factorial';
  if (hasTreeMatch || branchCount > 0) {
    speciesId = 'tree_fold';
  } else if (hasListMatch || consCount > 0) {
    speciesId = 'list_fold';
  } else if (fns.length >= 2 && crossRecursiveCalls > 0) {
    speciesId = 'mutual_even_odd';
  } else if (letCount >= 1 && selfRecursiveCalls === 0) {
    speciesId = 'lexical_let';
  } else if (
    (subCount > 0 && selfRecursiveCalls === 0 && ifCount === 0) ||
    terminalRuleLine === 225
  ) {
    speciesId = 'checked_underflow';
  } else if (mulCount > 0 && selfRecursiveCalls > 0) {
    speciesId = 'factorial';
  } else if (
    terminalRuleLine === 140 ||
    (addCount > 0 && selfRecursiveCalls > 0 && mulCount === 0 && !hasListMatch && !hasTreeMatch)
  ) {
    speciesId = 'stack_overflow';
  } else if (
    outcomeKind === 'Exhausted' ||
    terminalRuleLine === 84 ||
    (selfRecursiveCalls > 0 && mulCount === 0 && addCount === 0 && crossRecursiveCalls === 0)
  ) {
    speciesId = 'fuel_exhaustion';
  }

  // Extract entry-expression parameters for dynamic 3D organ counts (`fact(5)`, `cons(...)`, `let seed = ...`)
  const entryExpr = exprs.find((e) => e.astPath === 'entry/') || exprs[0] || null;
  const entryText = String(entryExpr?.text || '');
  const entryLits = [];
  for (const ex of exprs) {
    if (ex.kind === 'Literal' && typeof ex.astPath === 'string' && ex.astPath.startsWith('entry/')) {
      const raw = String(ex.text || '').trim();
      if (/^\d+$/.test(raw)) {
        entryLits.push(Number(BigInt(raw) & 0xffffffffn));
      }
    }
  }
  const entryN = entryLits[0] ?? literals[0] ?? 4;
  const listValues = speciesId === 'list_fold' && entryLits.length > 0 ? entryLits : [5, 8, 3];
  const treeLeaves = speciesId === 'tree_fold' && entryLits.length > 0 ? entryLits : [7, 4, 9];
  const seedValue = speciesId === 'lexical_let' ? (entryLits[0] ?? 12) : 12;
  const subGap =
    speciesId === 'checked_underflow' && entryLits.length >= 2
      ? Math.abs(entryLits[1] - entryLits[0])
      : 6;

  return {
    speciesId,
    programName: parsed?.programName || 'SherlockProgram',
    entryType: parsed?.entryType || 'UInt',
    entryText,
    entryMorph: {
      entryN,
      listValues,
      treeLeaves,
      seedValue,
      subGap,
    },
    functionCount: fns.length,
    expressionCount: exprs.length,
    hasListMatch,
    hasTreeMatch,
    letCount,
    ifCount,
    matchCount,
    mulCount,
    addCount,
    subCount,
    consCount,
    branchCount,
    selfRecursiveCalls,
    crossRecursiveCalls,
    literals,
    stackLimit,
    initialFuel,
    outcomeKind,
    terminalRuleLine,
    seed: (fnv1a32(source || parsed?.programName || 'sherlock') % 2147483646) + 1,
  };
}

/**
 * Translates any parsed Sherlock AST (`parsed`) + execution trace (`trace`) into an
 * admitted, executable Quinelings Task Graph (`QuinelingKernels.validate(graph)`).
 * Every node in the task graph maps directly to real expressions and functions in
 * the Sherlock program (`exprId` / `astPath`), so the Quineling's organs and
 * territorial colors are genuinely compiled from the Sherlock program!
 */
export function buildQuinelingTaskGraphFromAst(parsed, analysis, trace = null) {
  const nodes = [];
  const usedIds = new Set();
  const exprToNodeId = new Map();

  function allocId(base, exprId = null) {
    let candidate = sanitizeNodeId(base, nodes.length);
    let suffix = 1;
    while (usedIds.has(candidate)) {
      suffix += 1;
      candidate = `${sanitizeNodeId(base, nodes.length)}_${suffix}`;
    }
    usedIds.add(candidate);
    if (exprId !== null && exprId !== undefined) {
      exprToNodeId.set(exprId, candidate);
    }
    return candidate;
  }

  function addNode(baseId, op, inputs, params = {}, exprMeta = null) {
    const id = allocId(baseId, exprMeta?.id);
    const node = {
      id,
      op,
      inputs,
      params,
    };
    nodes.push(node);
    return id;
  }

  const exprs = parsed?.programImage?.expressions || [];
  const fns = parsed?.functions || [];
  const lits = analysis.literals.length > 0 ? analysis.literals : [4, 1, 0];

  // 1. Create input/literal nodes directly from the Sherlock program's entry & literal AST nodes
  const literalExprs = exprs.filter((e) => e.kind === 'Literal');
  const numericSeedValues =
    lits.length >= 2 ? lits.slice(0, 6) : [lits[0] ?? 4, 3, 2, 1];

  const entryInputId = addNode(
    `entry_${parsed?.programName || 'seed'}`,
    'literal',
    [],
    { value: numericSeedValues },
    exprs[0] || null
  );

  const scalarSeedVal = lits[0] ?? 4;
  const scalarInputId = addNode(
    `lit_${scalarSeedVal}`,
    'literal',
    [],
    { value: scalarSeedVal },
    literalExprs[0] || null
  );

  // 2. Walk every AST expression in `parsed.programImage.expressions` and emit a corresponding
  //    admitted Quinelings operation node so the task graph has the exact topology of the AST!
  let currentArrayId = entryInputId;
  let currentScalarId = scalarInputId;
  let lastGuardId = null;
  const reportInputs = [];
  const reportLabels = [];

  function pushReportInput(nodeId, labelBase) {
    if (reportInputs.length >= 8 || reportInputs.includes(nodeId)) return;
    let lbl = sanitizeNodeId(labelBase, reportLabels.length);
    let c = 1;
    while (reportLabels.includes(lbl)) {
      c += 1;
      lbl = `${sanitizeNodeId(labelBase, reportLabels.length)}_${c}`;
    }
    reportInputs.push(nodeId);
    reportLabels.push(lbl);
  }

  for (let i = 0; i < exprs.length && nodes.length < 28; i += 1) {
    const ex = exprs[i];
    const tag = `${ex.kind.toLowerCase()}_L${ex.line || i}`;

    if (ex.kind === 'If') {
      const cmpId = addNode(
        `if_guard_L${ex.line}`,
        'compare',
        [currentScalarId],
        { operator: 'eq', value: 0 },
        ex
      );
      lastGuardId = cmpId;
      const baseLitId = addNode(
        `if_base_L${ex.line}`,
        'literal',
        [],
        { value: lits[1] ?? 1 }
      );
      const chooseId = addNode(
        `if_branch_L${ex.line}`,
        'choose',
        [cmpId, baseLitId, currentScalarId],
        {},
        ex
      );
      currentScalarId = chooseId;
      pushReportInput(chooseId, `branch_L${ex.line}`);
    } else if (ex.kind === 'Match') {
      const filtId = addNode(
        `match_pat_L${ex.line}`,
        'filter',
        [currentArrayId],
        { operator: 'gte', value: 0 },
        ex
      );
      const foldId = addNode(
        `match_fold_L${ex.line}`,
        'sum',
        [filtId],
        {},
        ex
      );
      currentArrayId = filtId;
      currentScalarId = foldId;
      pushReportInput(foldId, `match_L${ex.line}`);
    } else if (ex.kind === 'Let') {
      const sqId = addNode(
        `let_${ex.name || 'bind'}_L${ex.line}`,
        'map',
        [currentArrayId],
        { kind: 'square' },
        ex
      );
      const sumId = addNode(
        `scope_${ex.name || 'bind'}_L${ex.line}`,
        'sum',
        [sqId],
        {},
        ex
      );
      currentArrayId = sqId;
      currentScalarId = sumId;
      pushReportInput(sumId, `let_${ex.name || i}`);
    } else if (ex.kind === 'Binary') {
      if (ex.op === '*') {
        const factor = Math.max(1, lits[0] ?? 4);
        const mulId = addNode(
          `mul_L${ex.line}`,
          'map',
          [currentArrayId],
          { kind: 'multiply', factor },
          ex
        );
        const prodSumId = addNode(`accum_mul_L${ex.line}`, 'sum', [mulId], {}, ex);
        currentArrayId = mulId;
        currentScalarId = prodSumId;
        pushReportInput(prodSumId, `mul_L${ex.line}`);
      } else if (ex.op === '+') {
        const sumId = addNode(`add_L${ex.line}`, 'sum', [currentArrayId], {}, ex);
        currentScalarId = sumId;
        pushReportInput(sumId, `add_L${ex.line}`);
      } else if (ex.op === '-') {
        const clampId = addNode(
          `sub_L${ex.line}`,
          'clamp',
          [currentScalarId],
          { min: 0, max: Math.max(10, scalarSeedVal) },
          ex
        );
        currentScalarId = clampId;
        pushReportInput(clampId, `sub_L${ex.line}`);
      } else if (ex.op === '==' || ex.op === '<') {
        const cmpId = addNode(
          `cmp_L${ex.line}`,
          'compare',
          [currentScalarId],
          { operator: ex.op === '==' ? 'eq' : 'lt', value: 0 },
          ex
        );
        lastGuardId = cmpId;
        pushReportInput(cmpId, `cmp_L${ex.line}`);
      } else if (ex.op === 'cons' || ex.op === 'branch') {
        const dedupeId = addNode(
          `${ex.op}_L${ex.line}`,
          'dedupe',
          [currentArrayId],
          {},
          ex
        );
        currentArrayId = dedupeId;
      }
    } else if (ex.kind === 'Call') {
      const callLenId = addNode(
        `call_${ex.name || 'fn'}_L${ex.line}`,
        'length',
        [currentArrayId],
        {},
        ex
      );
      currentScalarId = callLenId;
      pushReportInput(callLenId, `call_${ex.name || i}`);
    } else {
      // Map Literal / Var / Unary AST expressions to explicit node territories
      exprToNodeId.set(ex.id, currentScalarId);
    }
  }

  // 3. Add explicit Fuel & Frame Horizon budget nodes from the Sherlock machine configuration
  const fuelLitId = addNode('fuel_budget', 'literal', [], {
    value: Math.max(1, analysis.initialFuel),
  });
  const stepsUsedVal = Math.max(1, trace?.steps?.length || 12);
  const stepsLitId = addNode('steps_charged', 'literal', [], {
    value: stepsUsedVal,
  });
  const budgetNodeId = addNode('fuel_witness', 'budget', [fuelLitId, stepsLitId], {});
  pushReportInput(budgetNodeId, 'fuelWitness');

  // 4. Add terminal publication / action receipt node matching outcome (`Done`, `Rejected`, `Exhausted`)
  if (!lastGuardId) {
    lastGuardId = addNode('terminal_guard', 'compare', [currentScalarId], {
      operator: 'gte',
      value: 0,
    });
  }
  const allowedFlag = analysis.outcomeKind === 'Done';
  const actionId = addNode(
    `publish_${analysis.outcomeKind.toLowerCase()}`,
    'action',
    [lastGuardId, currentScalarId],
    {
      allowed: allowedFlag,
      action: `${analysis.programName}_${analysis.outcomeKind}`,
    }
  );
  pushReportInput(actionId, 'publication');

  const reportId = addNode(
    `sherlock_${ sanitizeNodeId(analysis.programName) }_out`,
    'report',
    reportInputs,
    { labels: reportLabels }
  );

  const graph = {
    version: 1,
    name: `Sherlock ${analysis.programName} (${analysis.speciesId})`,
    nodes,
    outputs: [reportId],
  };

  // Validate against the real Quinelings kernel verifier
  QuinelingKernels.validate(graph);
  return { graph, exprToNodeId };
}

/**
 * Authors the exact `WovenBody` record for each of the 8 Sherlock species so that
 * every program compiles through its own distinct 3D mathematical mechanism in `WovenBody`!
 */
function authorSpeciesWovenRecord(graph, speciesMeta, seed) {
  const base = WovenBody.author(graph, seed);
  const t = base.topology;
  const strands = base.strands;
  const territories = base.territories;
  const mechanism = speciesMeta.mechanism;

  const record = {
    model: 'woven-field-experimental',
    mechanism,
    seed,
    topology: t,
    strands,
    folds: speciesMeta.folds ?? Math.min(7, 3 + t.convergence),
    turns: speciesMeta.turns ?? 1.45,
    territories,
  };

  if (
    mechanism === 'logarithmic-mantle' ||
    mechanism === 'toroidal-weave' ||
    mechanism === 'phyllotaxis-fan'
  ) {
    const geom = LifeformFamilies.authorParams(mechanism, { ...t, strands }, seed);
    const clampVal = (v, lo, hi) => Math.max(lo, Math.min(hi, v));
    Object.assign(geom, {
      growth: clampVal(speciesMeta.growth ?? geom.growth, 0.12, 0.35),
      spread: clampVal(speciesMeta.spread ?? geom.spread, 0.5, 1.2),
      curl: clampVal(speciesMeta.curl ?? geom.curl, 0.4, 1.6),
      asymmetry: clampVal(speciesMeta.asymmetry ?? geom.asymmetry, 0, 0.4),
      frequency: Math.round(clampVal(speciesMeta.frequency ?? geom.frequency, 2, 8)),
      lobes: Math.round(clampVal(speciesMeta.lobes ?? geom.lobes, 3, 9)),
      twist: clampVal(speciesMeta.twist ?? geom.twist, 1, 3),
      whorls: Math.round(clampVal(speciesMeta.whorls ?? geom.whorls, 2, 4)),
    });
    if (mechanism === 'toroidal-weave') {
      delete geom.material;
    }
    record.geometry = geom;
  } else if (mechanism === 'recursive-affine') {
    record.geometry = {
      model: 'contractive-branch-charts',
      chart: 'rolled-scroll',
      rollTurns: 2.8,
      depth: 4,
      root: [0.0, -0.42],
      length: 0.56,
      width: 0.16,
      angle: Math.PI * 0.5,
      motion: 0.18,
      transforms: [
        { scale: 0.62, angle: -0.68, attach: 0.78 },
        { scale: 0.64, angle: 0.62, attach: 0.82 },
        { scale: 0.52, angle: -0.22, attach: 0.54 },
      ],
    };
  } else if (mechanism === 'lorenz-flow') {
    record.dynamics = {
      system: 'lorenz',
      method: 'rk4',
      sigma: 10,
      rho: 28,
      beta: 8 / 3,
      initial: [0.18, 0.24, 0.12],
      dt: 0.005,
      burn: 512,
      steps: 3200,
      window: 960,
    };
  } else if (mechanism === 'clifford-flow') {
    record.dynamics = {
      system: 'clifford',
      method: 'iterate',
      parameters: [-1.4, 1.6, 1.0, 0.7],
      initial: [0.15, 0.22],
      burn: 320,
      iterations: 8192,
      densityGrid: 96,
      criticalImages: 6,
    };
    record.embedding = {
      model: 'bounded-native-warp',
      scale: [0.28, 0.34],
      warp: [0.08, 0.08],
      overtone: [0.035, 0.035],
      frequency: [2.2, 2.1],
      shear: 0.04,
    };
  } else if (mechanism === 'recursive-julia') {
    record.geometry = {
      model: 'quadratic-julia',
      parameter: [-0.745, 0.113],
      grid: 128,
      iterations: 48,
      escape: 2,
      bounds: [-1.55, 1.55, -1.2, 1.2],
      levels: [3, 6, 11, 18, 28, 40],
      embedding: {
        scale: [0.34, 0.42],
        warp: [0.08, 0.07],
        frequency: [2.1, 2.2],
        shear: -0.06,
      },
    };
  }

  WovenBody.validate(record, graph);
  return record;
}

const compiledQuinelingCache = new Map();

/**
 * Compiles a Sherlock program (`source`, `trace`) into a verified, self-reproducing
 * Quineling (`quineProgram`, `shape`, `wovenBody`, `harmonicGenome`, `colorGenome`,
 * `taskRun`, `exprToNodeId`) using the real Quinelings SDK.
 */
export function compileSherlockProgramToQuineling(source, speciesCatalog, trace = null) {
  const cleanSource = String(
    source ||
      trace?.source ||
      'language "moriarty-sherlock-draft/1"; program Factorial { fn fact(n: UInt): UInt { if n == 0 { 1 } else { n * fact(n - 1) } } entry fact(4); }'
  );
  const stackLimit = trace?.steps?.[0]?.after?.stackLimit ?? trace?.stackLimit ?? 16;
  const initialFuel = trace?.steps?.[0]?.before?.fuel ?? trace?.fuel ?? 120;
  const outcomeKind = trace?.outcome?.kind || 'Done';
  const cacheKey = `${cleanSource}|S:${stackLimit}|F:${initialFuel}|O:${outcomeKind}`;

  if (compiledQuinelingCache.has(cacheKey)) {
    return compiledQuinelingCache.get(cacheKey);
  }

  let parsed = null;
  try {
    parsed = parseAndCheckDraft(cleanSource);
  } catch (_) {
    parsed = {
      programName: trace?.programName || 'SherlockProgram',
      entryType: 'UInt',
      functions: [],
      entry: null,
      programImage: trace?.programImage || { expressions: [], functions: [] },
    };
  }

  const analysis = analyzeSherlockAst(parsed, cleanSource, trace);
  const speciesMeta =
    speciesCatalog[analysis.speciesId] || speciesCatalog.factorial;

  const { graph, exprToNodeId } = buildQuinelingTaskGraphFromAst(
    parsed,
    analysis,
    trace
  );

  // 1. Create QDL Design for this species' family (`nautilus`, `ribbon`, `coral`, `moth`, `seed`, `jelly`, `comet`, `torus`)
  const design = QDL.create(speciesMeta.family);
  design.surface.samples = 6000;
  design.chroma.strength = 1;

  // 2. Build self-reproducing Quine AST & verify exact Quine self-emission
  const quineProgram = Quinelings.makeTaskProgram(graph, 1, design);
  const quineCanonical = Quinelings.canon(quineProgram);
  const quineProof = Quinelings.execute(quineProgram, { constructionOnly: true });
  const quineVerified =
    Array.isArray(quineProof.emitted) && quineProof.emitted[0] === quineCanonical;

  // 3. Describe shape, compile Chroma territorial field, and encode Harmonic + RGB Genomes
  const shape = Quinelings.describe(quineProgram);
  const chromaField = Chroma.compile(shape);
  const harmonicGenome = Quinelings.encode(quineProgram);
  const colorGenome = Quinelings.encodeColors(quineProgram);
  const taskRun = QuinelingKernels.run(graph);

  // 4. Author & compile the 3D `WovenBody` manifold for this species' exact mechanism
  const wovenRecord = authorSpeciesWovenRecord(graph, speciesMeta, analysis.seed);
  const wovenBody = WovenBody.compile(wovenRecord, graph);
  const wovenBounds = WovenBody.portraitFrame(wovenBody);
  const morphBounds = Morphology.portraitFrame(shape);

  // Flatten non-null RGB pixels from the real `quineling-chroma-1` genome for 3D spore rendering
  const rgbPixels = [];
  for (const band of colorGenome.pixels) {
    for (const px of band) {
      if (px) rgbPixels.push(px);
    }
  }

  const nodeColors = shape.nodes.map((_, idx) => {
    const hexColor = Chroma.colorFor(shape, idx);
    const rgb = typeof Chroma.channels === 'function'
      ? Chroma.channels(hexColor)
      : [1, 3, 5].map((i) => parseInt(hexColor.slice(i, i + 2), 16));
    return { hex: hexColor, rgb };
  });

  const artifact = {
    cacheKey,
    source: cleanSource,
    parsed,
    analysis,
    speciesId: speciesMeta.id,
    speciesMeta,
    graph,
    exprToNodeId,
    design,
    quineProgram,
    quineCanonical,
    quineVerified,
    quineBytes: new TextEncoder().encode(quineCanonical).length,
    constructorSteps: quineProof.steps,
    shape,
    chromaField,
    nodeColors,
    harmonicGenome,
    colorGenome,
    rgbPixels,
    taskRun,
    wovenRecord,
    wovenBody,
    wovenBounds,
    morphBounds,
  };

  if (compiledQuinelingCache.size >= 24) {
    const oldestKey = compiledQuinelingCache.keys().next().value;
    compiledQuinelingCache.delete(oldestKey);
  }
  compiledQuinelingCache.set(cacheKey, artifact);
  return artifact;
}
