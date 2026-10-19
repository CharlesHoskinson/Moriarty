// ============================================================================
// src/designers/designer8_quineling.js
// Designer 8: Authentic `nursery.html` Quinelings SDK & `WovenBody` 3D Lifeforms
// Compiles actual Sherlock programs (`moriarty-sherlock-draft/1` AST + traces)
// through the enhanced upstream Quinelings SDK (`Quinelings.makeTaskProgram`,
// `QDL.fromGraph`, `WovenBody.author`, `Morphology`, `Chroma`, `LifeformFamilies`)
// and renders the genuine mathematical `WovenBody` shapes (`logarithmic-mantle`,
// `pleated-braid`, `recursive-affine`, `lorenz-flow`, `phyllotaxis-fan`,
// `clifford-flow`, `recursive-julia`, `toroidal-weave`) in 3D inside the hollow
// processor chamber — with program-driven variation in shape, motion, and color.
// ============================================================================
import * as THREE from 'three/webgpu';
import {
  clamp,
  lerp,
  tagInspectable,
} from './theme.js';
import { WovenBody, QDL } from '../engine/quinelingsSdk.js';
import {
  fnv1a32,
  compileSherlockProgramToQuineling,
} from '../engine/sherlockQuinelingCompiler.js';
import { createPersistentPhaseRing } from '../engine/functionalStructures.js';

export { fnv1a32 };

const TAU = Math.PI * 2;
const PHI_GOLDEN_ANGLE = Math.PI * (3 - Math.sqrt(5));

// Detect mobile / coarse-pointer devices so Android/iOS CPUs & GPUs run at locked 60 FPS
const IS_MOBILE_RUNTIME =
  typeof navigator !== 'undefined' &&
  (/Android|iPhone|iPad|iPod|Mobile/i.test(navigator.userAgent || '') ||
    (typeof window !== 'undefined' &&
      typeof window.matchMedia === 'function' &&
      window.matchMedia('(max-width: 900px), (hover: none) and (pointer: coarse)').matches));

// Spatial Anchor & Strict Envelope Constants inside the 3.12 × 3.12 × 3.40 Vault
export const QUINELING_CORE_Y = -2.12;
export const QUINELING_MAX_RADIUS = 0.42;

const TISSUE_BUDGET = IS_MOBILE_RUNTIME ? 3072 : 8192;
const MAX_RIDGE_SEGMENTS = IS_MOBILE_RUNTIME ? 8192 : 28672;
const MAX_RIDGE_VERTICES = MAX_RIDGE_SEGMENTS * 2;
const PHASE_BUCKET_COUNT = IS_MOBILE_RUNTIME ? 24 : 36;

const SDK_RIBBON_COUNT = 24;
const SDK_RIBBON_SEGMENTS = 24;
const SDK_RIBBON_VERTEX_COUNT = SDK_RIBBON_COUNT * SDK_RIBBON_SEGMENTS * 2;

const MAX_ANCHOR_NODES = 16;
const MAX_DEPENDENCY_ARCS = 24;
const ARC_SEGMENTS = 12;
const MAX_ARC_VERTICES = MAX_DEPENDENCY_ARCS * (ARC_SEGMENTS - 1) * 2;

const SPORE_COUNT = 64;
const TENDRIL_CURVE_COUNT = 16;
const TENDRIL_SEGMENTS = 10;
const TENDRIL_VERTEX_COUNT = TENDRIL_CURVE_COUNT * TENDRIL_SEGMENTS * 2;

const ROLES_1_HEX = Object.freeze({
  inputCyan: 0x35d6c3,
  processBlue: 0x639bfa,
  decisionAmber: 0xedc653,
  quoteViolet: 0xb184ef,
  actionCoral: 0xf2757d,
  reportGreen: 0x2dd4bf,
  pearlCrest: 0xe6f8ed,
  faultCrimson: 0xe11d48,
  exhaustSlate: 0x64748b,
});

function quinticStep(z) {
  const t = clamp(z, 0, 1);
  return t * t * t * (t * (t * 6 - 15) + 10);
}

/**
 * Canonical 8-Program Quineling Species Profiles (`nursery.html` `WovenBody` mechanisms)
 * Every Sherlock program compiles through the upstream SDK (`WovenBody.author` + `QDL.fromGraph`)
 * into its authentic mathematical `WovenBody` lifeform.
 */
export const QUINELING_SPECIES = Object.freeze({
  factorial: {
    id: 'factorial',
    speciesName: 'Logarithmic Mantle',
    binomial: 'Spiralis multiplicata',
    commonTitle: 'Logarithmic Mantle // Coiled Ribbon Chart',
    family: 'nautilus',
    mechanism: 'logarithmic-mantle',
    symmetryOrder: 'Two-Parameter Logarithmic Shell Chart + Jacobian Compression',
    gesture: 'glide',
    primaryHex: 0x72d9e2,
    secondaryHex: 0xe6c66a,
    accentCss: '#72d9e2',
    description:
      'Genuine WovenBody logarithmic-mantle lifeform authored from convergent multiplicative accumulation, with projected-compression Jacobian caustics and spiral parameter filaments.',
  },
  list_fold: {
    id: 'list_fold',
    speciesName: 'Pleated Braid',
    binomial: 'Vitta catenaria',
    commonTitle: 'Pleated Braid // Folded Ribbon Manifold',
    family: 'ribbon',
    mechanism: 'pleated-braid',
    symmetryOrder: 'Multi-Strand Pleated Braid Immersion + Fold Ridges',
    gesture: 'gather',
    primaryHex: 0x6ad4a0,
    secondaryHex: 0xbb91ed,
    accentCss: '#6ad4a0',
    description:
      'Genuine WovenBody pleated-braid lifeform authored from sequential cons/nil pipeline folds, weaving core and peripheral ribbon lanes with harmonic fold crests.',
  },
  tree_fold: {
    id: 'tree_fold',
    speciesName: 'Recursive Affine',
    binomial: 'Dendron bifurcata',
    commonTitle: 'Recursive Affine // Contractive Branch Charts',
    family: 'coral',
    mechanism: 'recursive-affine',
    symmetryOrder: 'Contractive Rolled-Scroll Affine Branch Charts + Jacobian Caustics',
    gesture: 'unfurl',
    primaryHex: 0x88d89f,
    secondaryHex: 0xefaa73,
    accentCss: '#88d89f',
    description:
      'Genuine WovenBody recursive-affine lifeform authored from hierarchical branch/leaf tree recursion, with contractive rolled-scroll branch charts and exact signed Jacobian caustics.',
  },
  mutual_even_odd: {
    id: 'mutual_even_odd',
    speciesName: 'Lorenz Strange Attractor',
    binomial: 'Bimorpha paritatis',
    commonTitle: 'Lorenz Flow // RK4 Strange Attractor',
    family: 'moth',
    mechanism: 'lorenz-flow',
    symmetryOrder: 'Classical RK4 Lorenz Attractor (σ=10, ρ=28, β=8/3) + Frenet Ribbons',
    gesture: 'hover',
    primaryHex: 0xbb91ed,
    secondaryHex: 0x72d9e2,
    accentCss: '#bb91ed',
    description:
      'Genuine WovenBody lorenz-flow lifeform authored from coupled is_even/is_odd parity alternation, integrating a two-wing RK4 Lorenz orbit with Frenet-frame ribbon pleats.',
  },
  lexical_let: {
    id: 'lexical_let',
    speciesName: 'Phyllotaxis Fan',
    binomial: 'Granum phyllotaxis',
    commonTitle: 'Phyllotaxis Fan // Golden-Angle Whorls',
    family: 'bloom',
    mechanism: 'phyllotaxis-fan',
    symmetryOrder: 'Golden-Angle (137.5°) Phyllotaxis Ribbon Whorls + Jacobian Compression',
    gesture: 'gather',
    primaryHex: 0xb5e681,
    secondaryHex: 0x899be8,
    accentCss: '#b5e681',
    description:
      'Genuine WovenBody phyllotaxis-fan lifeform authored from nested lexical let-scope chains, arranging two-parameter ribbon charts along the golden angle with projected-compression caustics.',
  },
  checked_underflow: {
    id: 'checked_underflow',
    speciesName: 'Clifford Strange Attractor',
    binomial: 'Umbrella fracturae',
    commonTitle: 'Clifford Flow // Critical Caustic Web',
    family: 'jelly',
    mechanism: 'clifford-flow',
    symmetryOrder: 'Native Clifford Recurrence Map + Forward-Iterated Critical Curves',
    gesture: 'hover',
    primaryHex: 0xeb90ba,
    secondaryHex: 0x77bce9,
    accentCss: '#eb90ba',
    description:
      'Genuine WovenBody clifford-flow lifeform authored from guarded subtractive boundary checks, sampling a 2D Clifford attractor density field and its forward-mapped Jacobian zero-determinant critical curves.',
  },
  stack_overflow: {
    id: 'stack_overflow',
    speciesName: 'Quadratic Julia Manifold',
    binomial: 'Cauda julia',
    commonTitle: 'Recursive Julia // Escape-Level Contours',
    family: 'comet',
    mechanism: 'recursive-julia',
    symmetryOrder: 'Complex Quadratic Recurrence z → z² + c + Marching-Squares Contours',
    gesture: 'glide',
    primaryHex: 0xefaa73,
    secondaryHex: 0xbb91ed,
    accentCss: '#efaa73',
    description:
      'Genuine WovenBody recursive-julia lifeform authored from divergent self-recursive call fanout, tracing smoothed escape-time tissue and marching-squares level-set contours of z → z² + c.',
  },
  fuel_exhaustion: {
    id: 'fuel_exhaustion',
    speciesName: 'Toroidal Weave',
    binomial: 'Annulus toroidalis',
    commonTitle: 'Toroidal Weave // Poloidal Ribbon Field',
    family: 'torus',
    mechanism: 'toroidal-weave',
    symmetryOrder: 'Multi-Lobe Toroidal Ribbon Immersion + Parameter Filaments',
    gesture: 'gather',
    primaryHex: 0x77bce9,
    secondaryHex: 0xe6c66a,
    accentCss: '#77bce9',
    description:
      'Genuine WovenBody toroidal-weave lifeform authored from bounded effectful loop schedules, wrapping multi-strand poloidal ribbons around an asymmetric breathing torus.',
  },
});

const PRESET_ALIAS_TO_ID = Object.freeze({
  factorial: 'factorial',
  list_fold: 'list_fold',
  list_sum: 'list_fold',
  tree_fold: 'tree_fold',
  tree_sum: 'tree_fold',
  mutual_even_odd: 'mutual_even_odd',
  lexical_let: 'lexical_let',
  let_shadow: 'lexical_let',
  checked_underflow: 'checked_underflow',
  checked_sub_trap: 'checked_underflow',
  stack_overflow: 'stack_overflow',
  fuel_exhaustion: 'fuel_exhaustion',
});

export function resolveCompiledQuineling(stepState = {}) {
  const source =
    stepState.source ||
    stepState.trace?.source ||
    'language "moriarty-sherlock-draft/1"; program Factorial { fn fact(n: UInt): UInt { if n == 0 { 1 } else { n * fact(n - 1) } } entry fact(4); }';
  return compileSherlockProgramToQuineling(
    source,
    QUINELING_SPECIES,
    stepState.trace || null
  );
}

export function resolveQuinelingSpecies(stepState = {}) {
  const rawId =
    stepState.presetId ||
    stepState.trace?.presetId ||
    stepState.programId ||
    '';
  const mappedId = PRESET_ALIAS_TO_ID[rawId];
  if (mappedId && QUINELING_SPECIES[mappedId] && !stepState.source) {
    return QUINELING_SPECIES[mappedId];
  }
  const artifact = resolveCompiledQuineling(stepState);
  return artifact.speciesMeta || QUINELING_SPECIES.factorial;
}

let _lastTelemetryKey = '';
let _lastTelemetryResult = null;

/**
 * Computes a concise live morphogenesis telemetry object for UI inspector (`#telemetryQdlMorph`)
 */
export function getQuinelingTelemetry(stepState = {}) {
  const artifact = resolveCompiledQuineling(stepState);
  const stepIndex = stepState.stepIndex ?? stepState.step?.index ?? 0;
  const telemetryKey = `${artifact.cacheKey}|step:${stepIndex}`;
  if (telemetryKey === _lastTelemetryKey && _lastTelemetryResult) {
    return _lastTelemetryResult;
  }

  const species = artifact.speciesMeta;
  const step = stepState.step ?? null;
  const ruleLine = step?.ruleLine ?? 80;
  const ruleId = step?.ruleId || `A${String(ruleLine).padStart(3, '0')}`;
  const frames = step?.after?.frames ?? [];
  const frameCount = frames.length;
  const beforeFrameCount = step?.before?.frameCount ?? frameCount;
  const activeCalls = step?.after?.activeCalls ?? [];
  const callDepth = activeCalls.length;
  const controlKind = step?.after?.control?.kind || step?.controlKind || 'Eval';
  const rhythmMode = artifact.design?.motion?.rhythm?.mode || 'breath-pulse';

  let morphGesture = `RHYTHM ${rhythmMode.toUpperCase()}`;
  let morphCss = artifact.nodeColors?.[0]?.hex || species.accentCss;

  if (ruleLine === 80 || ruleId === 'A080') {
    morphGesture = `CROWN BLOOM · ${artifact.wovenRecord.mechanism.toUpperCase()} DONE`;
    morphCss = '#2dd4bf';
  } else if (ruleLine === 82 || ruleLine === 140 || ruleLine === 225) {
    morphGesture = `CAUSTIC FAULT LOCK · ${ruleId}`;
    morphCss = '#e11d48';
  } else if (ruleLine === 84 || ruleId === 'A084') {
    morphGesture = 'TOROIDAL EMBER FADE · 0F';
    morphCss = '#64748b';
  } else if (ruleLine === 149 || ruleLine === 145 || ruleLine === 177) {
    morphGesture = `CHIRAL PHASE LOCK · ${ruleId}`;
    morphCss = '#e6c66a';
  } else if (ruleLine === 223) {
    morphGesture = `16-LIMB ALU CONVERGENCE · ${rhythmMode.toUpperCase()}`;
    morphCss = '#72d9e2';
  } else if (frameCount > beforeFrameCount || ruleLine === 135) {
    morphGesture = `UNFURLING PATCH #${frameCount} · DEPTH ${callDepth}`;
    morphCss = artifact.nodeColors?.[frameCount % Math.max(1, artifact.nodeColors.length)]?.hex || species.accentCss;
  } else if (frameCount < beforeFrameCount || controlKind === 'Return') {
    morphGesture = `MANIFOLD SYNTHESIS · ${frameCount} OPEN`;
    morphCss = '#6ad4a0';
  } else {
    morphGesture = `${artifact.wovenRecord.mechanism.toUpperCase()} · ${rhythmMode.toUpperCase()}`;
  }

  _lastTelemetryKey = telemetryKey;
  _lastTelemetryResult = {
    speciesId: species.id,
    speciesName: species.speciesName,
    binomial: species.binomial,
    commonTitle: species.commonTitle,
    family: artifact.design?.family || species.family,
    mechanism: artifact.wovenRecord?.mechanism || species.mechanism,
    symmetryOrder: species.symmetryOrder,
    morphGesture,
    morphCss,
    quineVerified: artifact.quineVerified,
    quineBytes: artifact.quineBytes,
    nodeCount: artifact.graph.nodes.length,
  };
  return _lastTelemetryResult;
}

let sharedTissueSpriteTexture = null;
function getTissueSpriteTexture() {
  if (sharedTissueSpriteTexture) return sharedTissueSpriteTexture;
  if (typeof document === 'undefined') return null;
  const canvas = document.createElement('canvas');
  canvas.width = 64;
  canvas.height = 64;
  const ctx = canvas.getContext('2d');
  const grad = ctx.createRadialGradient(32, 32, 1, 32, 32, 31);
  grad.addColorStop(0.0, 'rgba(255, 255, 255, 1.0)');
  grad.addColorStop(0.32, 'rgba(245, 252, 255, 0.82)');
  grad.addColorStop(0.68, 'rgba(210, 240, 250, 0.26)');
  grad.addColorStop(1.0, 'rgba(0, 0, 0, 0.0)');
  ctx.fillStyle = grad;
  ctx.fillRect(0, 0, 64, 64);
  sharedTissueSpriteTexture = new THREE.CanvasTexture(canvas);
  sharedTissueSpriteTexture.needsUpdate = true;
  return sharedTissueSpriteTexture;
}

function writeRibbonStrip(positions, colors, normals, curveIndex, segments, pointsArray, halfWidth, r, g, b) {
  const baseVertex = curveIndex * segments * 2;
  for (let s = 0; s < segments; s += 1) {
    const p = pointsArray[s];
    const pNext = pointsArray[Math.min(segments - 1, s + 1)];
    const pPrev = pointsArray[Math.max(0, s - 1)];

    let tx = pNext.x - pPrev.x;
    let ty = pNext.y - pPrev.y;
    let tz = pNext.z - pPrev.z;
    const tLen = Math.hypot(tx, ty, tz) || 1;
    tx /= tLen;
    ty /= tLen;
    tz /= tLen;

    let bx = -tz;
    let by = 0;
    let bz = tx;
    const bLen = Math.hypot(bx, by, bz);
    if (bLen < 1e-4) {
      bx = 1;
      by = 0;
      bz = 0;
    } else {
      bx /= bLen;
      bz /= bLen;
    }

    const nx = ty * bz - tz * by;
    const ny = tz * bx - tx * bz;
    const nz = tx * by - ty * bx;

    const u = s / Math.max(1, segments - 1);
    const taper = Math.sin(u * Math.PI) * 0.72 + 0.28;
    const w = halfWidth * taper;

    const v0 = (baseVertex + s * 2) * 3;
    const v1 = (baseVertex + s * 2 + 1) * 3;

    positions[v0] = p.x - bx * w;
    positions[v0 + 1] = p.y - by * w;
    positions[v0 + 2] = p.z - bz * w;

    positions[v1] = p.x + bx * w;
    positions[v1 + 1] = p.y + by * w;
    positions[v1 + 2] = p.z + bz * w;

    normals[v0] = nx;
    normals[v0 + 1] = ny;
    normals[v0 + 2] = nz;
    normals[v1] = nx;
    normals[v1 + 1] = ny;
    normals[v1 + 2] = nz;

    const glow = 0.68 + 0.32 * Math.sin(u * Math.PI);
    colors[v0] = r * glow;
    colors[v0 + 1] = g * glow;
    colors[v0 + 2] = b * glow;
    colors[v1] = r * glow;
    colors[v1 + 1] = g * glow;
    colors[v1 + 2] = b * glow;
  }
}

function buildRibbonIndices(curveCount, segments) {
  const indexArray = [];
  for (let c = 0; c < curveCount; c += 1) {
    const base = c * segments * 2;
    for (let s = 0; s < segments - 1; s += 1) {
      const i0 = base + s * 2;
      const i1 = i0 + 1;
      const i2 = i0 + 2;
      const i3 = i0 + 3;
      indexArray.push(i0, i2, i1, i1, i2, i3);
    }
  }
  return indexArray;
}

/**
 * Evaluates the program-authored QDL `motion.rhythm` phase modulation (`nursery.js` / `visual-capsule.js`)
 * so each program moves with its own distinct rhythm (`breath-pulse`, `crest-wave`,
 * `asymmetric-glide`, `harmonic-overtone`).
 */
function evaluateProgramRhythmPhase(design, basePhase) {
  const mr = design?.motion?.rhythm;
  if (!mr) return basePhase;
  const rate = Number.isFinite(mr.rate) ? mr.rate : 1.0;
  const breath = Number.isFinite(mr.breath) ? mr.breath : 0.14;
  const wave = Number.isFinite(mr.wave) ? mr.wave : 0.12;
  const asym = Number.isFinite(mr.asymmetry) ? mr.asymmetry : 0.1;
  const overtone = Number.isFinite(mr.overtone) ? mr.overtone : 0.08;
  const t = basePhase * rate;

  switch (mr.mode) {
    case 'crest-wave':
      return t + wave * Math.sin(2 * t) + overtone * Math.cos(3 * t);
    case 'asymmetric-glide':
      return t + asym * Math.sin(t + asym * Math.cos(t)) + 0.5 * wave * Math.sin(2 * t);
    case 'harmonic-overtone':
      return t + breath * Math.sin(t) + overtone * Math.sin(3 * t + 0.6);
    case 'breath-pulse':
    default:
      return t + breath * Math.sin(t) + 0.35 * wave * Math.cos(2 * t);
  }
}

/**
 * Normalizes a 3D `WovenBody` point `(x, y, z)` into the Hollow Well atrium envelope
 * (`r <= QUINELING_MAX_RADIUS = 0.44`) while preserving exact `nursery.html` proportions
 * and giving 2.5D manifolds (`clifford-flow`, `recursive-julia`) volumetric 3D depth.
 */
function normalizeWovenCoord(x, y, z, bounds, mechanism, outVec) {
  const cx = bounds?.cx || 0;
  const cy = bounds?.cy || 0;
  const cz = bounds?.cz || 0;
  const span = Math.max(0.32, bounds?.width || 1.2, bounds?.height || 1.2);
  const scale = (QUINELING_MAX_RADIUS * 1.88) / span;

  const nx = (x - cx) * scale;
  // Flip Y because nursery.html / WovenBody uses 2D screen coordinates (+Y downward)
  const ny = -(y - cy) * scale;
  let nz = (z - cz) * scale;

  // Give planar/shallow attractors subtle 3D relief under camera orbit without altering their 2D projection
  if (mechanism === 'clifford-flow' || mechanism === 'recursive-julia') {
    nz = nz * 2.2 + 0.045 * Math.sin(nx * 5.2 + ny * 4.1);
  } else if (mechanism === 'recursive-affine') {
    nz *= 1.5;
  }

  outVec.set(nx, ny, nz);
  return outVec;
}

const MECHANISM_VELOCITY_GAIN = Object.freeze({
  'logarithmic-mantle': 2.2,
  'pleated-braid': 1.85,
  'recursive-affine': 2.0,
  'lorenz-flow': 2.45,
  'phyllotaxis-fan': 2.35,
  'clifford-flow': 2.1,
  'recursive-julia': 2.05,
  'toroidal-weave': 2.3,
});

function computeSpatialWaveAngle(nx, ny, nz, mechanism) {
  const r = Math.hypot(nx, ny);
  const theta = Math.atan2(ny, nx);
  switch (mechanism) {
    case 'logarithmic-mantle':
      return r * 13.5 - theta * 2.0;
    case 'pleated-braid':
      return nx * 9.5 + ny * 5.5;
    case 'recursive-affine':
      return r * 11.0 + theta - ny * 4.0;
    case 'lorenz-flow':
      return r * 11.0 + (nx >= 0 ? 0 : 1.57);
    case 'phyllotaxis-fan':
      return r * 15.0 - theta * 5.0;
    case 'clifford-flow':
      return nx * 9.5 + ny * 7.5;
    case 'recursive-julia':
      return r * 14.0 + theta * 2.0;
    case 'toroidal-weave':
    default:
      return r * 12.5 + theta * 3.0 + nz * 4.0;
  }
}

/**
 * Returns (or lazily initializes) the functional `PersistentPhaseRing` memo table on `artifact`.
 * Each bucket stores pre-normalized Structure-of-Arrays (SoA) tissue & ridge geometry over S^1 = [0, 2π),
 * plus precomputed spatial wave sin/cos and exponential density alpha so per-frame 60/120fps advection
 * uses zero transcendental calls inside the vertex loop!
 */
function ensureArtifactPhaseRing(artifact, frameReuse, vecA, vecB) {
  if (artifact._phaseRing && artifact._phaseRingBudget === TISSUE_BUDGET) {
    return artifact._phaseRing;
  }

  const species = artifact.speciesMeta || QUINELING_SPECIES.factorial;
  const mechanism = artifact.wovenRecord?.mechanism || species.mechanism;
  const bounds = artifact.wovenBounds || { cx: 0, cy: 0, cz: 0, width: 1.2, height: 1.2, depth: 0.4 };
  const nodeColors = artifact.nodeColors || [];
  const nodeCount = Math.max(1, nodeColors.length);
  const baseGain = artifact.wovenBody?.cloud
    ? 0.24
    : artifact.wovenBody?.familyAdapter
      ? 0.34
      : 0.48;

  artifact._phaseRingBudget = TISSUE_BUDGET;
  artifact._phaseRing = createPersistentPhaseRing(PHASE_BUCKET_COUNT, (bucketPhase) => {
    const rawFrame = WovenBody.frame(artifact.wovenBody, bucketPhase, {
      budget: TISSUE_BUDGET,
      crests: true,
      reuse: frameReuse,
    });

    const pts = rawFrame.points;
    const owners = rawFrame.owners;

    const baseTissuePos = new Float32Array(TISSUE_BUDGET * 3);
    const baseTissueWave = new Float32Array(TISSUE_BUDGET * 2);
    const baseTissueExpAlpha = new Float32Array(TISSUE_BUDGET);
    const baseTissuePearl = new Float32Array(TISSUE_BUDGET);
    const baseTissueOwner = new Uint16Array(TISSUE_BUDGET);

    for (let i = 0; i < TISSUE_BUDGET; i += 1) {
      const p4 = i * 4;
      const p3 = i * 3;
      const p2 = i * 2;
      normalizeWovenCoord(pts[p4], pts[p4 + 1], pts[p4 + 2], bounds, mechanism, vecA);
      const nx = vecA.x;
      const ny = vecA.y;
      const nz = vecA.z;
      baseTissuePos[p3] = nx;
      baseTissuePos[p3 + 1] = ny;
      baseTissuePos[p3 + 2] = nz;

      const kAngle = computeSpatialWaveAngle(nx, ny, nz, mechanism);
      baseTissueWave[p2] = Math.sin(kAngle);
      baseTissueWave[p2 + 1] = Math.cos(kAngle);

      const rawPtAlpha = pts[p4 + 3] || 0.04;
      baseTissueExpAlpha[i] = 1 - Math.exp(-2.2 * rawPtAlpha);
      baseTissuePearl[i] = rawPtAlpha > 0.24 ? clamp((rawPtAlpha - 0.24) * 0.55, 0, 0.28) : 0;
      baseTissueOwner[i] = owners[i] % nodeCount;
    }

    const ridges = rawFrame.ridges || [];
    let totalRawSegments = 0;
    for (let rIdx = 0; rIdx < ridges.length; rIdx += 1) {
      const len = ridges[rIdx].line?.length || 0;
      if (len >= 2) totalRawSegments += len - 1;
    }
    const stepStride = totalRawSegments > MAX_RIDGE_SEGMENTS ? 2 : 1;

    // First pass: count active segments
    let segCount = 0;
    for (let rIdx = 0; rIdx < ridges.length && segCount < MAX_RIDGE_SEGMENTS; rIdx += 1) {
      const line = ridges[rIdx].line;
      if (!Array.isArray(line) || line.length < 2) continue;
      for (let j = 0; j < line.length - 1 && segCount < MAX_RIDGE_SEGMENTS; j += stepStride) {
        const nextJ = Math.min(line.length - 1, j + stepStride);
        const rawA0 = Number.isFinite(line[j].alpha) ? line[j].alpha : 0.2;
        const rawA1 = Number.isFinite(line[nextJ].alpha) ? line[nextJ].alpha : 0.2;
        if (Math.max(rawA0, rawA1) < 0.012) continue;
        segCount += 1;
      }
    }

    const baseRidgePos = new Float32Array(segCount * 6);
    const baseRidgeWave = new Float32Array(segCount * 4);
    const baseRidgeAlpha = new Float32Array(segCount * 2);
    const baseRidgeOwner = new Uint16Array(segCount);

    let segCursor = 0;
    for (let rIdx = 0; rIdx < ridges.length && segCursor < segCount; rIdx += 1) {
      const line = ridges[rIdx].line;
      if (!Array.isArray(line) || line.length < 2) continue;
      const isParamFilament = ridges[rIdx].material === 'parameter-filament';

      for (let j = 0; j < line.length - 1 && segCursor < segCount; j += stepStride) {
        const nextJ = Math.min(line.length - 1, j + stepStride);
        const p0 = line[j];
        const p1 = line[nextJ];

        const rawA0 = Number.isFinite(p0.alpha) ? p0.alpha : 0.2;
        const rawA1 = Number.isFinite(p1.alpha) ? p1.alpha : 0.2;
        if (Math.max(rawA0, rawA1) < 0.012) continue;

        const focus0 = isParamFilament ? Math.pow(clamp(rawA0, 0, 1), 1.45) : clamp(rawA0, 0, 1);
        const focus1 = isParamFilament ? Math.pow(clamp(rawA1, 0, 1), 1.45) : clamp(rawA1, 0, 1);

        normalizeWovenCoord(p0.x, p0.y, p0.z || 0, bounds, mechanism, vecA);
        normalizeWovenCoord(p1.x, p1.y, p1.z || 0, bounds, mechanism, vecB);

        const vBase = segCursor * 6;
        baseRidgePos[vBase] = vecA.x;
        baseRidgePos[vBase + 1] = vecA.y;
        baseRidgePos[vBase + 2] = vecA.z;
        baseRidgePos[vBase + 3] = vecB.x;
        baseRidgePos[vBase + 4] = vecB.y;
        baseRidgePos[vBase + 5] = vecB.z;

        const wBase = segCursor * 4;
        const k0 = computeSpatialWaveAngle(vecA.x, vecA.y, vecA.z, mechanism);
        const k1 = computeSpatialWaveAngle(vecB.x, vecB.y, vecB.z, mechanism);
        baseRidgeWave[wBase] = Math.sin(k0);
        baseRidgeWave[wBase + 1] = Math.cos(k0);
        baseRidgeWave[wBase + 2] = Math.sin(k1);
        baseRidgeWave[wBase + 3] = Math.cos(k1);

        baseRidgeAlpha[segCursor * 2] = clamp(focus0 * baseGain, 0.004, 0.72);
        baseRidgeAlpha[segCursor * 2 + 1] = clamp(focus1 * baseGain, 0.004, 0.72);
        baseRidgeOwner[segCursor] =
          Number.isInteger(p0.owner) && p0.owner >= 0
            ? p0.owner % nodeCount
            : rIdx % nodeCount;

        segCursor += 1;
      }
    }

    return {
      baseTissuePos,
      baseTissueWave,
      baseTissueExpAlpha,
      baseTissuePearl,
      baseTissueOwner,
      baseRidgePos,
      baseRidgeWave,
      baseRidgeAlpha,
      baseRidgeOwner,
      ridgeSegCount: segCursor,
    };
  });

  return artifact._phaseRing;
}

/**
 * Builds the 3D `nursery.html` `WovenBody` Quineling Subsystem (`Designer 8`).
 * Replaces all cartoon animal sculptures with the genuine mathematical `WovenBody`
 * tissue field, caustic/crest filaments, gossamer manifold ribbons, `WovenBody.anchor`
 * operation nodes, and `quineling-chroma-1` RGB genome spores.
 */
export function createQuinelingSubsystem() {
  const group = new THREE.Group();
  group.name = 'Designer8_QuinelingCore';

  const coreRoot = new THREE.Group();
  coreRoot.name = 'QuinelingOrganismRoot';
  coreRoot.position.set(0, QUINELING_CORE_Y, 0);
  group.add(coreRoot);

  // Studio illumination for the translucent mathematical tissue
  const corePointLight = new THREE.PointLight(0x72d9e2, 0.85, 2.4, 1.5);
  corePointLight.position.set(0, 0.06, 0.08);
  coreRoot.add(corePointLight);

  const rimPointLight = new THREE.PointLight(0x9ad7ff, 0.45, 2.4, 1.5);
  rimPointLight.position.set(0.22, 0.18, 0.26);
  coreRoot.add(rimPointLight);

  // Backwards-compatible species sub-groups map for inspection & telemetry hooks
  const speciesGroups = {};
  for (const spKey of Object.keys(QUINELING_SPECIES)) {
    const sg = new THREE.Group();
    sg.name = `WovenSpecies_${spKey}`;
    sg.visible = spKey === 'factorial';
    coreRoot.add(sg);
    speciesGroups[spKey] = sg;
  }

  // ==========================================================================
  // 1. PRIMARY `WovenBody` GOSSAMER TISSUE CLOUD (`THREE.Points`, 8192 samples)
  // ==========================================================================
  const tissueGeom = new THREE.BufferGeometry();
  const tissuePositions = new Float32Array(TISSUE_BUDGET * 3);
  const tissueColors = new Float32Array(TISSUE_BUDGET * 3);
  tissueGeom.setAttribute('position', new THREE.BufferAttribute(tissuePositions, 3));
  tissueGeom.setAttribute('color', new THREE.BufferAttribute(tissueColors, 3));

  const tissueMat = new THREE.PointsMaterial({
    size: 0.012,
    map: getTissueSpriteTexture(),
    vertexColors: true,
    transparent: true,
    opacity: 0.78,
    blending: THREE.AdditiveBlending,
    depthWrite: false,
    sizeAttenuation: true,
  });
  const _raycastSphere = new THREE.Sphere(new THREE.Vector3(0, QUINELING_CORE_Y, 0), QUINELING_MAX_RADIUS * 0.82);
  const _raycastHitPoint = new THREE.Vector3();
  const NOOP_RAYCAST = () => {};

  const tissuePointsMesh = new THREE.Points(tissueGeom, tissueMat);
  tissuePointsMesh.name = 'WovenBody_TissueCloud';
  tissuePointsMesh.frustumCulled = false;
  tissuePointsMesh.raycast = function fastWovenBodyRaycast(raycaster, intersects) {
    if (!this.visible) return;
    _raycastSphere.center.setFromMatrixPosition(this.matrixWorld);
    _raycastSphere.radius = QUINELING_MAX_RADIUS * 0.82;
    if (raycaster.ray.intersectSphere(_raycastSphere, _raycastHitPoint)) {
      intersects.push({
        distance: raycaster.ray.origin.distanceTo(_raycastHitPoint),
        point: _raycastHitPoint.clone(),
        object: this,
      });
    }
  };
  tagInspectable(tissuePointsMesh, {
    type: 'quineling-tissue',
    id: 'woven-body-tissue-cloud',
    title: 'WovenBody Mathematical Tissue Field',
    subtitle: 'nursery.html // WovenBody.frame',
    details: '8,192 real-time points sampled from WovenBody.frame(wovenBody, phase) and colored by Chroma.colorFor(shape, owner).',
  });
  coreRoot.add(tissuePointsMesh);

  // ==========================================================================
  // 2. PRIMARY `WovenBody` CAUSTIC & CREST FILAMENTS (`THREE.LineSegments`)
  // ==========================================================================
  const ridgeLineGeom = new THREE.BufferGeometry();
  const ridgeLinePositions = new Float32Array(MAX_RIDGE_VERTICES * 3);
  const ridgeLineColors = new Float32Array(MAX_RIDGE_VERTICES * 3);
  ridgeLineGeom.setAttribute('position', new THREE.BufferAttribute(ridgeLinePositions, 3));
  ridgeLineGeom.setAttribute('color', new THREE.BufferAttribute(ridgeLineColors, 3));
  ridgeLineGeom.setDrawRange(0, 0);

  const ridgeLineMat = new THREE.LineBasicMaterial({
    vertexColors: true,
    transparent: true,
    opacity: 0.72,
    blending: THREE.AdditiveBlending,
    depthWrite: false,
  });
  const ridgeLineSegments = new THREE.LineSegments(ridgeLineGeom, ridgeLineMat);
  ridgeLineSegments.name = 'WovenBody_CausticRidges';
  ridgeLineSegments.frustumCulled = false;
  ridgeLineSegments.raycast = NOOP_RAYCAST;
  tagInspectable(ridgeLineSegments, {
    type: 'quineling-ridges',
    id: 'woven-body-caustic-ridges',
    title: 'WovenBody Caustic & Crest Filaments',
    subtitle: 'nursery.html // frame.ridges',
    details: 'Exact mathematical ridges from WovenBody.frame: critical curves, Julia escape contours, Jacobian caustics, and parameter filaments.',
  });
  coreRoot.add(ridgeLineSegments);

  // ==========================================================================
  // 3. 3D `WovenBody` GOSSAMER RIBBON STRIPS (`sdkManifoldRibbonMesh`)
  // ==========================================================================
  const sdkRibbonGeom = new THREE.BufferGeometry();
  const sdkRibbonPositions = new Float32Array(SDK_RIBBON_VERTEX_COUNT * 3);
  const sdkRibbonColors = new Float32Array(SDK_RIBBON_VERTEX_COUNT * 3);
  const sdkRibbonNormals = new Float32Array(SDK_RIBBON_VERTEX_COUNT * 3);
  sdkRibbonGeom.setAttribute('position', new THREE.BufferAttribute(sdkRibbonPositions, 3));
  sdkRibbonGeom.setAttribute('color', new THREE.BufferAttribute(sdkRibbonColors, 3));
  sdkRibbonGeom.setAttribute('normal', new THREE.BufferAttribute(sdkRibbonNormals, 3));
  sdkRibbonGeom.setIndex(buildRibbonIndices(SDK_RIBBON_COUNT, SDK_RIBBON_SEGMENTS));

  const sdkRibbonMat = new THREE.MeshPhysicalMaterial({
    color: 0xffffff,
    vertexColors: true,
    emissive: new THREE.Color(0x030b12),
    emissiveIntensity: 0.18,
    roughness: 0.38,
    metalness: 0.02,
    transparent: true,
    opacity: 0.09,
    side: THREE.DoubleSide,
    depthWrite: false,
  });
  const sdkManifoldRibbonMesh = new THREE.Mesh(sdkRibbonGeom, sdkRibbonMat);
  sdkManifoldRibbonMesh.name = 'WovenBody_GossamerRibbons';
  sdkManifoldRibbonMesh.frustumCulled = false;
  sdkManifoldRibbonMesh.visible = false;
  sdkManifoldRibbonMesh.raycast = NOOP_RAYCAST;
  tagInspectable(sdkManifoldRibbonMesh, {
    type: 'quineling-ribbons',
    id: 'woven-body-gossamer-ribbons',
    title: 'WovenBody 3D Gossamer Ribbons',
    subtitle: 'WovenBody // 3D Ribbon Chart',
    details: 'Translucent 3D ribbon strips tracing the primary mathematical ridges of the active WovenBody manifold.',
  });
  coreRoot.add(sdkManifoldRibbonMesh);

  // ==========================================================================
  // 4. OPERATION ANCHOR NODES (`WovenBody.anchor`) & DEPENDENCY ARCS
  // ==========================================================================
  const anchorNodeGeom = new THREE.SphereGeometry(0.0055, 10, 8);
  const anchorNodeMat = new THREE.MeshStandardMaterial({
    color: 0xffffff,
    emissive: new THREE.Color(0xffffff),
    emissiveIntensity: 0.65,
    roughness: 0.25,
    metalness: 0.1,
    transparent: true,
    opacity: 0.62,
  });
  const anchorNodeMesh = new THREE.InstancedMesh(anchorNodeGeom, anchorNodeMat, MAX_ANCHOR_NODES);
  anchorNodeMesh.instanceMatrix.setUsage(THREE.DynamicDrawUsage);
  anchorNodeMesh.name = 'WovenBody_OperationAnchors';
  anchorNodeMesh.frustumCulled = false;
  anchorNodeMesh.visible = false;
  anchorNodeMesh.raycast = NOOP_RAYCAST;
  tagInspectable(anchorNodeMesh, {
    type: 'quineling-anchors',
    id: 'woven-body-operation-anchors',
    title: 'Quinelings Operation Territory Anchors',
    subtitle: 'WovenBody.anchor // Quinelings.describe',
    details: 'Operation nodes positioned on the 3D manifold via WovenBody.anchor(wovenBody, nodeId, phase) and lit when active in the Sherlock CEK trace.',
  });
  coreRoot.add(anchorNodeMesh);

  const depArcGeom = new THREE.BufferGeometry();
  const depArcPositions = new Float32Array(MAX_ARC_VERTICES * 3);
  const depArcColors = new Float32Array(MAX_ARC_VERTICES * 3);
  depArcGeom.setAttribute('position', new THREE.BufferAttribute(depArcPositions, 3));
  depArcGeom.setAttribute('color', new THREE.BufferAttribute(depArcColors, 3));
  depArcGeom.setDrawRange(0, 0);

  const depArcMat = new THREE.LineBasicMaterial({
    vertexColors: true,
    transparent: true,
    opacity: 0.14,
    blending: THREE.AdditiveBlending,
    depthWrite: false,
  });
  const depArcLines = new THREE.LineSegments(depArcGeom, depArcMat);
  depArcLines.name = 'WovenBody_DependencyArcs';
  depArcLines.frustumCulled = false;
  depArcLines.visible = false;
  depArcLines.raycast = NOOP_RAYCAST;
  coreRoot.add(depArcLines);

  // ==========================================================================
  // 5. 64 RGB GENOME SPORES (`Quinelings.encodeColors` -> `quineling-chroma-1`)
  // ==========================================================================
  const sporeGeom = new THREE.OctahedronGeometry(0.004, 0);
  const sporeMat = new THREE.MeshStandardMaterial({
    color: 0xffffff,
    emissive: new THREE.Color(0xffffff),
    emissiveIntensity: 0.55,
    roughness: 0.3,
    metalness: 0.1,
    transparent: true,
    opacity: 0.45,
  });
  const sporeMesh = new THREE.InstancedMesh(sporeGeom, sporeMat, SPORE_COUNT);
  sporeMesh.instanceMatrix.setUsage(THREE.DynamicDrawUsage);
  sporeMesh.name = 'Quineling_ChromaGenomeSpores';
  sporeMesh.frustumCulled = false;
  sporeMesh.visible = false;
  sporeMesh.raycast = NOOP_RAYCAST;
  tagInspectable(sporeMesh, {
    type: 'quineling-spores',
    id: 'quineling-chroma-genome-spores',
    title: 'quineling-chroma-1 Exact Byte Spores',
    subtitle: 'Quinelings.encodeColors // quineling-chroma-1',
    details: '64 exact RGB byte triplets from Quinelings.encodeColors(quineProgram) drifting across the WovenBody manifold.',
  });
  coreRoot.add(sporeMesh);

  // ==========================================================================
  // 6. PERIMETER CHAMBER TENDRILS (Connecting Atrium Boundary to Rings/Floor)
  // ==========================================================================
  const tendrilGeom = new THREE.BufferGeometry();
  const tendrilPositions = new Float32Array(TENDRIL_VERTEX_COUNT * 3);
  const tendrilColors = new Float32Array(TENDRIL_VERTEX_COUNT * 3);
  const tendrilNormals = new Float32Array(TENDRIL_VERTEX_COUNT * 3);
  tendrilGeom.setAttribute('position', new THREE.BufferAttribute(tendrilPositions, 3));
  tendrilGeom.setAttribute('color', new THREE.BufferAttribute(tendrilColors, 3));
  tendrilGeom.setAttribute('normal', new THREE.BufferAttribute(tendrilNormals, 3));
  tendrilGeom.setIndex(buildRibbonIndices(TENDRIL_CURVE_COUNT, TENDRIL_SEGMENTS));

  const tendrilMat = new THREE.MeshPhysicalMaterial({
    color: 0xffffff,
    vertexColors: true,
    emissive: new THREE.Color(0x050c14),
    emissiveIntensity: 0.45,
    roughness: 0.4,
    metalness: 0.08,
    transparent: true,
    opacity: 0.28,
    side: THREE.DoubleSide,
    depthWrite: false,
  });
  const tendrilMesh = new THREE.Mesh(tendrilGeom, tendrilMat);
  tendrilMesh.name = 'Quineling_AtriumTendrils';
  tendrilMesh.frustumCulled = false;
  tendrilMesh.visible = false;
  tendrilMesh.raycast = NOOP_RAYCAST;
  coreRoot.add(tendrilMesh);

  // ==========================================================================
  // PREALLOCATED SCRATCH BUFFERS (ZERO PER-FRAME ALLOCATIONS)
  // ==========================================================================
  const _frameReuse = {
    points: new Float32Array(TISSUE_BUDGET * 4),
    owners: new Uint16Array(TISSUE_BUDGET),
  };
  const _sdkPts = Array.from({ length: SDK_RIBBON_SEGMENTS }, () => new THREE.Vector3());
  const _tendrilPts = Array.from({ length: TENDRIL_SEGMENTS }, () => new THREE.Vector3());
  const _anchorPositions = Array.from({ length: MAX_ANCHOR_NODES }, () => new THREE.Vector3());
  const _vecA = new THREE.Vector3();
  const _vecB = new THREE.Vector3();
  const _dummy = new THREE.Object3D();
  const _color = new THREE.Color();
  const _faultColor = new THREE.Color(0xe11d48);

  let dampedYOffset = 0;
  let accumPhase = 0;
  let stepSurgeVel = 0;
  let lastStepIndex = -1;
  let lastBucketLookupFrame = -10;
  let frameCounter = 0;
  let currentBucket = null;

  function update(stepState = {}, ambientArg = 0, dtArg = 0.016) {
    const ambient = stepState?.ambient ?? ambientArg;
    const dt = clamp(stepState?.dt ?? dtArg, 0.001, 0.05);
    const artifact = resolveCompiledQuineling(stepState);
    const species = artifact.speciesMeta || QUINELING_SPECIES.factorial;
    const speciesId = species.id;
    const mechanism = artifact.wovenRecord?.mechanism || species.mechanism;
    const bounds = artifact.wovenBounds || { cx: 0, cy: 0, cz: 0, width: 1.2, height: 1.2, depth: 0.4 };
    const design = artifact.design;

    // Toggle active species marker group
    for (const [key, sg] of Object.entries(speciesGroups)) {
      sg.visible = key === speciesId;
    }

    const step = stepState.step ?? null;
    const stepIndex = stepState.stepIndex ?? 0;
    const rawSub = clamp(stepState.sub ?? 1, 0, 1);
    const q = quinticStep(rawSub);
    const ruleLine = step?.ruleLine ?? 80;
    const ruleId = step?.ruleId || `A${String(ruleLine).padStart(3, '0')}`;
    const activeCalls = step?.after?.activeCalls ?? [];
    const callDepth = activeCalls.length;

    const preFuel = step?.before?.fuel ?? 20;
    const postFuel = step?.after?.fuel ?? preFuel;
    const initialFuel = Math.max(1, stepState.trace?.steps?.[0]?.before?.fuel ?? 20);
    const liveFuelRatio = clamp(lerp(preFuel, postFuel, q) / initialFuel, 0, 1);

    const isDone = ruleLine === 80 || ruleId === 'A080';
    const isFault = ruleLine === 82 || ruleLine === 140 || ruleLine === 225;
    const isExhausted = ruleLine === 84 || ruleId === 'A084';
    const isInPlaceRewrite = ruleLine === 149 || ruleLine === 145 || ruleLine === 177;

    // Smooth step-transition velocity surge (zero discontinuous phase jumps!)
    if (stepIndex !== lastStepIndex) {
      if (lastStepIndex !== -1) {
        stepSurgeVel = Math.min(2.6, stepSurgeVel + 0.95);
      }
      lastStepIndex = stepIndex;
    }
    stepSurgeVel *= Math.exp(-dt * 5.5);

    // Restore upstream QDL 24x phase timebase (`phaseRate * 24`) + mechanism velocity gain
    const rawPhaseRate = design?.motion?.phaseRate ?? 0.034;
    const qdlPhaseSpeed = rawPhaseRate < 0.15 ? rawPhaseRate * 24.0 : rawPhaseRate * 3.8;
    const mechGain = MECHANISM_VELOCITY_GAIN[mechanism] ?? 2.1;
    const fuelMod = isExhausted ? 0.28 : 0.72 + 0.48 * liveFuelRatio;
    const omega = qdlPhaseSpeed * mechGain * fuelMod + stepSurgeVel;

    accumPhase = (accumPhase + dt * omega) % (TAU * 64);
    const wovenPhase = evaluateProgramRhythmPhase(design, accumPhase);

    const rhythm = design?.motion?.rhythm;
    const breathAmp = (rhythm?.breath ?? 0.14) * 0.16;
    const asymTilt = (rhythm?.asymmetry ?? 0.1) * 0.26;

    const targetYOffset = clamp((callDepth - 1) * 0.014, -0.05, 0.06);
    dampedYOffset = lerp(dampedYOffset, targetYOffset, 1 - Math.exp(-dt * 8.0));

    coreRoot.position.y = QUINELING_CORE_Y + dampedYOffset + Math.sin(wovenPhase) * breathAmp;
    coreRoot.rotation.y =
      ambient * 0.34 +
      Math.sin(wovenPhase * 0.55) * 0.28 +
      (isInPlaceRewrite ? Math.sin(q * Math.PI) * 0.14 : 0);
    coreRoot.rotation.x = Math.cos(wovenPhase * 0.7) * asymTilt * 0.32;
    coreRoot.rotation.z = Math.sin(wovenPhase * 0.5) * asymTilt * 0.22;

    const breathScale = 1.0 + Math.sin(wovenPhase) * (rhythm?.breath ?? 0.12) * 0.15;
    coreRoot.scale.setScalar(breathScale);

    // Primary node color for core studio light
    const primaryNodeHex = artifact.nodeColors?.[0]?.hex || species.accentCss;
    _color.set(primaryNodeHex);
    if (isFault) _color.setHex(ROLES_1_HEX.faultCrimson);
    else if (isDone) _color.setHex(ROLES_1_HEX.reportGreen);
    corePointLight.color.copy(_color);
    corePointLight.intensity = isDone ? 1.15 : isFault ? 1.25 : isExhausted ? 0.35 : 0.82 + callDepth * 0.06;

    // Active AST expression -> Quinelings operation node ownership highlight
    const activeExprId =
      step?.after?.control?.exprId ??
      step?.before?.control?.exprId ??
      null;
    const activeNodeId =
      activeExprId !== null && artifact.exprToNodeId?.has(activeExprId)
        ? artifact.exprToNodeId.get(activeExprId)
        : null;
    const activeNodeEntry =
      activeNodeId && artifact.nodesById?.has(activeNodeId)
        ? artifact.nodesById.get(activeNodeId)
        : null;
    const activeNodeIdx = activeNodeEntry
      ? activeNodeEntry.index
      : stepIndex % Math.max(1, artifact.graph.nodes.length);

    // ========================================================================
    // 1. PERSISTENT FUNCTIONAL PHASE RING (`PersistentPhaseRing` over S^1)
    //    Cold buckets are solved at most once every 3 frames during the first
    //    2π cycle; once warm, `phaseRing.getNearest(wovenPhase)` is an O(1)
    //    zero-allocation array lookup forever!
    // ========================================================================
    frameCounter += 1;
    const phaseRing = ensureArtifactPhaseRing(artifact, _frameReuse, _vecA, _vecB);
    const targetBucketIdx = phaseRing.bucketIndexForPhase(wovenPhase);
    const isTargetWarm = phaseRing.buckets[targetBucketIdx] !== null;

    if (isTargetWarm || !currentBucket || frameCounter - lastBucketLookupFrame >= 3) {
      if (!isTargetWarm) {
        lastBucketLookupFrame = frameCounter;
      }
      currentBucket = phaseRing.getNearest(wovenPhase).bucket;
    }

    const baseTissuePos = currentBucket.baseTissuePos;
    const baseTissueWave = currentBucket.baseTissueWave;
    const baseTissueExpAlpha = currentBucket.baseTissueExpAlpha;
    const baseTissuePearl = currentBucket.baseTissuePearl;
    const baseTissueOwner = currentBucket.baseTissueOwner;
    const nodeColors = artifact.nodeColors || [];
    const nodeCount = Math.max(1, nodeColors.length);

    // Precompute per-frame trig for traveling wave sin(k - 2.5*phi) & cos(k - 2.5*phi)
    const sinPhi = Math.sin(wovenPhase * 2.5);
    const cosPhi = Math.cos(wovenPhase * 2.5);

    // Precompute per-node color tables once per frame (<= 16 entries instead of 36,000x!)
    const nodeBaseR = new Float32Array(nodeCount);
    const nodeBaseG = new Float32Array(nodeCount);
    const nodeBaseB = new Float32Array(nodeCount);
    const ridgeBaseR = new Float32Array(nodeCount);
    const ridgeBaseG = new Float32Array(nodeCount);
    const ridgeBaseB = new Float32Array(nodeCount);

    for (let n = 0; n < nodeCount; n += 1) {
      const rgb = nodeColors[n]?.rgb || [114, 217, 226];
      let tr = rgb[0] / 255;
      let tg = rgb[1] / 255;
      let tb = rgb[2] / 255;
      let rr = tr * 0.88 + 0.902 * 0.12;
      let rg = tg * 0.88 + 0.973 * 0.12;
      let rb = tb * 0.88 + 0.929 * 0.12;

      if (isFault) {
        tr = lerp(tr, 0.95, 0.72);
        tg *= 0.25;
        tb *= 0.32;
        rr = lerp(rr, 0.95, 0.72);
        rg *= 0.25;
        rb *= 0.32;
      } else if (isExhausted) {
        const tGray = (tr + tg + tb) * 0.333;
        tr = tGray * 0.55;
        tg = tGray * 0.6;
        tb = tGray * 0.68;
        const rGray = (rr + rg + rb) * 0.333;
        rr = rGray * 0.55;
        rg = rGray * 0.6;
        rb = rGray * 0.68;
      }
      nodeBaseR[n] = tr;
      nodeBaseG[n] = tg;
      nodeBaseB[n] = tb;
      ridgeBaseR[n] = rr;
      ridgeBaseG[n] = rg;
      ridgeBaseB[n] = rb;
    }

    // ========================================================================
    // 2. EVERY-FRAME 60/120FPS 3D TRAVELING-WAVE ADVECTION ON TISSUE CLOUD
    //    Zero transcendental calls inside the loop!
    // ========================================================================
    for (let i = 0; i < TISSUE_BUDGET; i += 1) {
      const p3 = i * 3;
      const p2 = i * 2;
      const bx = baseTissuePos[p3];
      const by = baseTissuePos[p3 + 1];
      const bz = baseTissuePos[p3 + 2];

      const sinK = baseTissueWave[p2];
      const cosK = baseTissueWave[p2 + 1];
      // sin(k - 2.5*phi) and cos(k - 2.5*phi)
      const waveS = sinK * cosPhi - cosK * sinPhi;
      const waveC = cosK * cosPhi + sinK * sinPhi;

      tissuePositions[p3] = bx + bx * 0.044 * waveS - by * 0.032 * waveC;
      tissuePositions[p3 + 1] = by + by * 0.044 * waveS + bx * 0.032 * waveC;
      tissuePositions[p3 + 2] = bz + 0.042 * waveS + 0.018 * waveC;

      const ownerIdx = baseTissueOwner[i];
      const gain = ownerIdx === activeNodeIdx ? 0.74 : 0.46;
      let a = baseTissueExpAlpha[i] * gain;
      if (a < 0.025) a = 0.025;
      else if (a > 0.78) a = 0.78;

      const boost = baseTissuePearl[i];
      const invBoost = 1 - boost;
      tissueColors[p3] = (nodeBaseR[ownerIdx] * invBoost + 0.902 * boost) * a;
      tissueColors[p3 + 1] = (nodeBaseG[ownerIdx] * invBoost + 0.973 * boost) * a;
      tissueColors[p3 + 2] = (nodeBaseB[ownerIdx] * invBoost + 0.929 * boost) * a;
    }
    tissueGeom.attributes.position.needsUpdate = true;
    tissueGeom.attributes.color.needsUpdate = true;

    // ========================================================================
    // 3. EVERY-FRAME 60/120FPS 3D TRAVELING-WAVE ADVECTION ON CAUSTIC RIDGES
    //    Zero transcendental calls inside the loop!
    // ========================================================================
    const baseRidgePos = currentBucket.baseRidgePos;
    const baseRidgeWave = currentBucket.baseRidgeWave;
    const baseRidgeAlpha = currentBucket.baseRidgeAlpha;
    const baseRidgeOwner = currentBucket.baseRidgeOwner;
    const ridgeSegCount = currentBucket.ridgeSegCount;

    for (let segIdx = 0; segIdx < ridgeSegCount; segIdx += 1) {
      const vBase = segIdx * 6;
      const wBase = segIdx * 4;
      const aBase = segIdx * 2;

      const bx0 = baseRidgePos[vBase];
      const by0 = baseRidgePos[vBase + 1];
      const bz0 = baseRidgePos[vBase + 2];
      const sinK0 = baseRidgeWave[wBase];
      const cosK0 = baseRidgeWave[wBase + 1];
      const waveS0 = sinK0 * cosPhi - cosK0 * sinPhi;
      const waveC0 = cosK0 * cosPhi + sinK0 * sinPhi;

      const bx1 = baseRidgePos[vBase + 3];
      const by1 = baseRidgePos[vBase + 4];
      const bz1 = baseRidgePos[vBase + 5];
      const sinK1 = baseRidgeWave[wBase + 2];
      const cosK1 = baseRidgeWave[wBase + 3];
      const waveS1 = sinK1 * cosPhi - cosK1 * sinPhi;
      const waveC1 = cosK1 * cosPhi + sinK1 * sinPhi;

      ridgeLinePositions[vBase] = bx0 + bx0 * 0.044 * waveS0 - by0 * 0.032 * waveC0;
      ridgeLinePositions[vBase + 1] = by0 + by0 * 0.044 * waveS0 + bx0 * 0.032 * waveC0;
      ridgeLinePositions[vBase + 2] = bz0 + 0.042 * waveS0 + 0.018 * waveC0;
      ridgeLinePositions[vBase + 3] = bx1 + bx1 * 0.044 * waveS1 - by1 * 0.032 * waveC1;
      ridgeLinePositions[vBase + 4] = by1 + by1 * 0.044 * waveS1 + bx1 * 0.032 * waveC1;
      ridgeLinePositions[vBase + 5] = bz1 + 0.042 * waveS1 + 0.018 * waveC1;

      const ownerIdx = baseRidgeOwner[segIdx];
      const activeBoost = ownerIdx === activeNodeIdx ? 1.28 : 1.0;

      let a0 = baseRidgeAlpha[aBase] * activeBoost;
      if (a0 > 0.72) a0 = 0.72;
      let a1 = baseRidgeAlpha[aBase + 1] * activeBoost;
      if (a1 > 0.72) a1 = 0.72;

      const baseR = ridgeBaseR[ownerIdx];
      const baseG = ridgeBaseG[ownerIdx];
      const baseB = ridgeBaseB[ownerIdx];

      ridgeLineColors[vBase] = baseR * a0;
      ridgeLineColors[vBase + 1] = baseG * a0;
      ridgeLineColors[vBase + 2] = baseB * a0;
      ridgeLineColors[vBase + 3] = baseR * a1;
      ridgeLineColors[vBase + 4] = baseG * a1;
      ridgeLineColors[vBase + 5] = baseB * a1;
    }
    ridgeLineGeom.setDrawRange(0, ridgeSegCount * 2);
    ridgeLineGeom.attributes.position.needsUpdate = true;
    ridgeLineGeom.attributes.color.needsUpdate = true;

    if (anchorNodeMesh.visible) {
      const shapeNodes = artifact.shape?.nodes || [];
      const activeAnchorCount = Math.min(MAX_ANCHOR_NODES, shapeNodes.length);
      anchorNodeMesh.count = activeAnchorCount;

      for (let nIdx = 0; nIdx < activeAnchorCount; nIdx += 1) {
        const node = shapeNodes[nIdx];
        let rawAnchor;
        try {
          rawAnchor = WovenBody.anchor(artifact.wovenBody, node.id, wovenPhase);
        } catch (_) {
          rawAnchor = { x: 0, y: 0, z: 0 };
        }
        normalizeWovenCoord(
          rawAnchor.x || 0,
          rawAnchor.y || 0,
          rawAnchor.z || 0,
          bounds,
          mechanism,
          _anchorPositions[nIdx]
        );

        const isActiveNode = nIdx === activeNodeIdx;
        _dummy.position.copy(_anchorPositions[nIdx]);
        _dummy.rotation.set(0, 0, 0);
        _dummy.scale.setScalar(isActiveNode ? 1.45 : 0.88);
        _dummy.updateMatrix();
        anchorNodeMesh.setMatrixAt(nIdx, _dummy.matrix);

        const cSpec = nodeColors[nIdx % nodeCount];
        const rgb = cSpec?.opRgb || cSpec?.rgb || [114, 217, 226];
        _color.setRGB(rgb[0] / 255, rgb[1] / 255, rgb[2] / 255);
        if (isActiveNode) _color.lerp(_faultColor, isFault ? 0.7 : 0.0);
        anchorNodeMesh.setColorAt(nIdx, _color);
      }
      anchorNodeMesh.instanceMatrix.needsUpdate = true;
      if (anchorNodeMesh.instanceColor) anchorNodeMesh.instanceColor.needsUpdate = true;
    }

    if (sporeMesh.visible) {
      const rgbPixels = artifact.rgbPixels || [];
      const outScale = isExhausted ? 0.92 : 1.04;

      for (let s = 0; s < SPORE_COUNT; s += 1) {
        const samplePt = Math.floor(((s * 61 + Math.floor(wovenPhase * 12)) % TISSUE_BUDGET)) * 3;
        const radialDrift = 0.014 * Math.sin(wovenPhase * 1.5 + s * PHI_GOLDEN_ANGLE);

        _dummy.position.set(
          tissuePositions[samplePt] * outScale + radialDrift,
          tissuePositions[samplePt + 1] * outScale + radialDrift * 0.6,
          tissuePositions[samplePt + 2] * outScale - radialDrift
        );
        _dummy.rotation.set(wovenPhase + s, wovenPhase * 0.7 + s, 0);
        _dummy.scale.setScalar(isDone ? 1.12 : isExhausted ? 0.52 : 0.84);
        _dummy.updateMatrix();
        sporeMesh.setMatrixAt(s, _dummy.matrix);

        const px = rgbPixels[s % Math.max(1, rgbPixels.length)];
        if (px && px.length >= 3) {
          _color.setRGB(px[0] / 255, px[1] / 255, px[2] / 255);
        } else {
          _color.setHex(species.primaryHex);
        }
        if (isFault) _color.lerp(_faultColor, 0.7);
        sporeMesh.setColorAt(s, _color);
      }
      sporeMesh.instanceMatrix.needsUpdate = true;
      if (sporeMesh.instanceColor) sporeMesh.instanceColor.needsUpdate = true;
    }
  }

  function dispose() {
    anchorNodeMesh.dispose();
    sporeMesh.dispose();
    const resources = [
      tissueGeom,
      tissueMat,
      ridgeLineGeom,
      ridgeLineMat,
      sdkRibbonGeom,
      sdkRibbonMat,
      anchorNodeGeom,
      anchorNodeMat,
      depArcGeom,
      depArcMat,
      sporeGeom,
      sporeMat,
      tendrilGeom,
      tendrilMat,
    ];
    for (let i = 0; i < resources.length; i += 1) {
      if (resources[i] && typeof resources[i].dispose === 'function') {
        resources[i].dispose();
      }
    }
    group.traverse((obj) => {
      if (obj.geometry && typeof obj.geometry.dispose === 'function') {
        obj.geometry.dispose();
      }
      if (obj.material) {
        if (Array.isArray(obj.material)) {
          for (let m = 0; m < obj.material.length; m += 1) {
            if (obj.material[m] && typeof obj.material[m].dispose === 'function') {
              obj.material[m].dispose();
            }
          }
        } else if (typeof obj.material.dispose === 'function') {
          obj.material.dispose();
        }
      }
    });
    currentBucket = null;
  }

  return {
    group,
    coreRoot,
    speciesGroups,
    tissuePointsMesh,
    ridgeLineSegments,
    sdkManifoldRibbonMesh,
    anchorNodeMesh,
    sporeMesh,
    update,
    dispose,
  };
}
