// ============================================================================
// src/designers/designer8_quineling.js
// Designer 8: Real Quinelings SDK Integration & 8 Distinct 3D Species Architectures
// Compiles actual Sherlock programs (`moriarty-sherlock-draft/1` AST + traces)
// through the upstream Quinelings SDK (`Quinelings.makeTaskProgram`, `QDL`,
// `WovenBody`, `Morphology`, `Chroma`, `LifeformFamilies`) and renders 8
// radically distinct 3D procedural lifeforms inside the hollow processor chamber.
// ============================================================================
import * as THREE from 'three/webgpu';
import {
  clamp,
  lerp,
  tagInspectable,
} from './theme.js';
import { WovenBody, Morphology } from '../engine/quinelingsSdk.js';
import {
  fnv1a32,
  compileSherlockProgramToQuineling,
} from '../engine/sherlockQuinelingCompiler.js';

export { fnv1a32 };

const TAU = Math.PI * 2;
const PHI_GOLDEN_ANGLE = Math.PI * (3 - Math.sqrt(5)); // ~2.399963 rad (137.50776°)

// Spatial Anchor & Strict Envelope Constants inside the 3.12 × 3.12 × 3.40 Vault
export const QUINELING_CORE_Y = -2.05;
export const QUINELING_MAX_RADIUS = 0.44; // Fits cleanly inside r <= 0.56 open atrium budget

const SPORE_COUNT = 64;
const SDK_RIBBON_COUNT = 16;
const SDK_RIBBON_SEGMENTS = 16;
const SDK_RIBBON_VERTEX_COUNT = SDK_RIBBON_COUNT * SDK_RIBBON_SEGMENTS * 2;

const TENDRIL_CURVE_COUNT = 16; // 8 floor root tendrils + 8 crown ring tendrils
const TENDRIL_SEGMENTS = 10;
const TENDRIL_VERTEX_COUNT = TENDRIL_CURVE_COUNT * TENDRIL_SEGMENTS * 2;

// Quinelings Fixed `roles-1` Semantic Dictionary (`docs/DESIGN-LANGUAGE.md`)
const ROLES_1_HEX = Object.freeze({
  inputCyan: 0x35d6c3,     // Cyan: inputs, literals & observations
  processBlue: 0x639bfa,   // Cerulean Blue: arithmetic, constructors & transforms
  decisionAmber: 0xedc653, // Amber Gold: If/Match gates & comparisons
  quoteViolet: 0xb184ef,   // Amethyst Violet: Let/Var lexical quotation & scope
  actionCoral: 0xf2757d,   // Coral Rose: recursive invocation & fault traps
  reportGreen: 0x2dd4bf,   // Emerald/Teal: verified return reports & Done(v)
  pearlCrest: 0xe6f8ed,    // Luminous pearl fold crest (Jacobian compression highlight)
  faultCrimson: 0xe11d48,  // Rule A082 / A140 / A225 trap spine color
  exhaustSlate: 0x64748b,  // Rule A084 fuel exhaustion ash-slate
});

/**
 * Fifth-order C2-continuous polynomial step H(z) = 6z^5 - 15z^4 + 10z^3
 */
function quinticStep(z) {
  const t = clamp(z, 0, 1);
  return t * t * t * (t * (t * 6 - 15) + 10);
}

/**
 * Canonical 8-Program Quineling Species Profiles (`qdl.js` + `WovenBody` mechanisms)
 * Every program maps to a distinct QDL family, a distinct `WovenBody` mechanism,
 * and a completely distinct 3D macro-anatomical silhouette.
 */
export const QUINELING_SPECIES = Object.freeze({
  factorial: {
    id: 'factorial',
    speciesName: 'Nautilus Arithmeticus',
    binomial: 'Spiralis multiplicata',
    commonTitle: 'Coiled Nautilus Shell // Spire',
    family: 'nautilus',
    mechanism: 'logarithmic-mantle',
    symmetryOrder: 'C1 Chiral Logarithmic Shell (r = r0·e^(b·θ))',
    gesture: 'glide',
    growth: 0.232,
    spread: 0.785,
    curl: 1.18,
    asymmetry: 0.21,
    frequency: 6,
    lobes: 5,
    twist: 2.5,
    whorls: 3,
    primaryHex: 0xf59e0b,
    secondaryHex: 0x10b981,
    accentCss: '#f59e0b',
    description:
      'Coiled 3D golden-ratio conispiral nautilus shell with pearlescent nacre tube and internal chamber septa (fact(N)..fact(0)) that lock from Amber into Gold on Rule A149.',
  },
  list_fold: {
    id: 'list_fold',
    speciesName: 'Taenia Cons-Catenata',
    binomial: 'Vitta catenaria',
    commonTitle: '3-Pod Siphonophore // Silk Ribbon',
    family: 'ribbon',
    mechanism: 'pleated-braid',
    symmetryOrder: 'C1 Horizontal Undulating 3-Zooid Siphonophore',
    gesture: 'gather',
    folds: 5,
    turns: 1.45,
    primaryHex: 0x06b6d4,
    secondaryHex: 0xa855f7,
    accentCss: '#06b6d4',
    description:
      'Wide horizontal undulating S-wave siphonophore silk membrane bearing bioluminescent cons(5, 8, 3) zooid bells that fire sequentially on Rule A214 & A220.',
  },
  tree_fold: {
    id: 'tree_fold',
    speciesName: 'Corallium Arboris',
    binomial: 'Dendron bifurcata',
    commonTitle: 'Dichotomous Binary Coral // Dendrite',
    family: 'coral',
    mechanism: 'recursive-affine',
    symmetryOrder: 'D2 Asymmetric Y-Branching Coral Tree',
    gesture: 'unfurl',
    folds: 6,
    turns: 1.65,
    primaryHex: 0x10b981,
    secondaryHex: 0xf59e0b,
    accentCss: '#10b981',
    description:
      'Rooted 3D bifurcating binary coral tree (Root Trunk -> West leaf(7) limb + East branch(leaf(4), leaf(9)) sub-limbs) crowned with glowing polyp rosettes at the leaf tips.',
  },
  mutual_even_odd: {
    id: 'mutual_even_odd',
    speciesName: 'Lepidoptera Alternans',
    binomial: 'Bimorpha paritatis',
    commonTitle: 'Twin-Winged Luna Moth // Lorenz Dipole',
    family: 'moth',
    mechanism: 'lorenz-flow',
    symmetryOrder: 'C2 Bilateral Twin-Winged Moth + Figure-8 Lorenz Orbit',
    gesture: 'hover',
    folds: 5,
    turns: 1.8,
    primaryHex: 0xa855f7,
    secondaryHex: 0x06b6d4,
    accentCss: '#a855f7',
    description:
      'Wide bilateral 2-winged Luna Moth (Left Cyan Wing = is_even, Right Violet Wing = is_odd) flapping in counter-phase inside a 3D Figure-8 RK4 Lorenz Strange Attractor tube.',
  },
  lexical_let: {
    id: 'lexical_let',
    speciesName: 'PhylloCube Umbra',
    binomial: 'Granum cubicum',
    commonTitle: '3-Nested Crystal Cubes // 24-Petal Lotus',
    family: 'seed',
    mechanism: 'phyllotaxis-fan',
    symmetryOrder: 'C_phi 24-Petal Golden-Angle Lotus + 3 Nested Cubes',
    gesture: 'gather',
    growth: 0.181,
    spread: 0.84,
    curl: 1.487,
    asymmetry: 0.2,
    frequency: 6,
    lobes: 5,
    twist: 2.5,
    whorls: 3,
    primaryHex: 0x8b5cf6,
    secondaryHex: 0xfbbf24,
    accentCss: '#8b5cf6',
    description:
      'Three concentric tilted crystal cubes (seed=12, sq=144, cb=1728) cradled inside an upward-blooming 24-petal golden-angle (137.5°) lotus rosette.',
  },
  checked_underflow: {
    id: 'checked_underflow',
    speciesName: 'Medusa Subtractor',
    binomial: 'Umbrella fracturae',
    commonTitle: 'Medusa Jellyfish Bell // 16 Guard Tentacles',
    family: 'jelly',
    mechanism: 'clifford-flow',
    symmetryOrder: 'C16 Hemispherical Medusa Bell + 16 Trailing Tentacles',
    gesture: 'hover',
    folds: 6,
    turns: 1.5,
    primaryHex: 0xf43f5e,
    secondaryHex: 0xf59e0b,
    accentCss: '#f43f5e',
    description:
      'Pulsing translucent hemispherical Medusa jellyfish umbrella dome at the top of the core with 16 sinuous trailing UInt128 byte-limb tentacles that snap crimson on Rule A225 underflow.',
  },
  stack_overflow: {
    id: 'stack_overflow',
    speciesName: 'Cometa Abyssi',
    binomial: 'Cauda quintus',
    commonTitle: 'Ballistic Comet // Horizon Shock Rings',
    family: 'comet',
    mechanism: 'recursive-julia',
    symmetryOrder: 'C5 Diagonal Comet Lance + Pancaked Ceiling Rings',
    gesture: 'glide',
    folds: 6,
    turns: 1.75,
    primaryHex: 0xf97316,
    secondaryHex: 0xe11d48,
    accentCss: '#fb7185',
    description:
      'Steeply angled ballistic comet head and tapered plasma tail firing upward into horizontal Julia-contour shockwave rings that pancake against the crimson stackLimit ceiling barrier.',
  },
  fuel_exhaustion: {
    id: 'fuel_exhaustion',
    speciesName: 'Cyclopora Exhaurire',
    binomial: 'Annulus quattuordecim',
    commonTitle: 'Hollow 14-Segment Tokamak // Braided Torus',
    family: 'torus',
    mechanism: 'toroidal-weave',
    symmetryOrder: 'C14 Hollow Open-Center (1,3) Braided Torus Ring',
    gesture: 'gather',
    growth: 0.228,
    spread: 0.767,
    curl: 1.414,
    asymmetry: 0.185,
    frequency: 6,
    lobes: 5,
    twist: 2.5,
    whorls: 3,
    primaryHex: 0x38bdf8,
    secondaryHex: 0xf59e0b,
    accentCss: '#38bdf8',
    description:
      'Wide, completely hollow open-center doughnut ring divided into radial fuel-cell segments wrapped in a (1,3) poloidal braid that extinguishes cell by cell as fuel drains to 0.',
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

/**
 * Resolves the compiled Quineling artifact (`compileSherlockProgramToQuineling`)
 * for the current `stepState`, ensuring the actual Sherlock source code (`factorial`,
 * `list_fold`, `tree_fold`, `mutual_even_odd`, `lexical_let`, `checked_underflow`,
 * `stack_overflow`, `fuel_exhaustion`, or custom edits) feeds the real Quinelings SDK.
 */
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

/**
 * Resolves the deterministic Quineling Species profile for any preset or custom Sherlock program.
 */
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

/**
 * Computes a concise live morphogenesis telemetry object for UI inspector (`#telemetryQdlMorph`)
 */
export function getQuinelingTelemetry(stepState = {}) {
  const artifact = resolveCompiledQuineling(stepState);
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

  let morphGesture = 'BREATHING AT REST';
  let morphCss = species.accentCss;

  if (ruleLine === 80 || ruleId === 'A080') {
    morphGesture = 'CROWN BLOOM · DONE';
    morphCss = '#0d9488';
  } else if (ruleLine === 82 || ruleLine === 140 || ruleLine === 225) {
    morphGesture = `FAULT SPINE LOCK · ${ruleId}`;
    morphCss = '#e11d48';
  } else if (ruleLine === 84 || ruleId === 'A084') {
    morphGesture = 'CENTRIPETAL EMBER FADE · 0F';
    morphCss = '#64748b';
  } else if (ruleLine === 149 || ruleLine === 145 || ruleLine === 177) {
    morphGesture = `IN-PLACE CHIRAL TWIST · ${ruleId}`;
    morphCss = '#d97706';
  } else if (ruleLine === 223) {
    morphGesture = '16-LIMB ALU CONVERGENCE';
    morphCss = '#0284c7';
  } else if (frameCount > beforeFrameCount || ruleLine === 135) {
    morphGesture = `UNFURLING ORGAN #${frameCount} · DEPTH ${callDepth}`;
    morphCss = species.accentCss;
  } else if (frameCount < beforeFrameCount || controlKind === 'Return') {
    morphGesture = `SYNTHESIS RETRACT · ${frameCount} OPEN`;
    morphCss = '#0d9488';
  } else {
    morphGesture = `ACTIVE ${controlKind.toUpperCase()} · ${artifact.graph.nodes.length} QDL NODES`;
  }

  return {
    speciesId: species.id,
    speciesName: species.speciesName,
    binomial: species.binomial,
    commonTitle: species.commonTitle,
    family: species.family,
    mechanism: species.mechanism,
    symmetryOrder: species.symmetryOrder,
    morphGesture,
    morphCss,
    quineVerified: artifact.quineVerified,
    quineBytes: artifact.quineBytes,
    nodeCount: artifact.graph.nodes.length,
  };
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

    const taper = Math.sin((s / Math.max(1, segments - 1)) * Math.PI) * 0.65 + 0.35;
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

    const glow = 0.72 + 0.28 * Math.sin((s / Math.max(1, segments - 1)) * Math.PI);
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
 * Helper to align a unit cylinder (`CylinderGeometry(1, 1, 1, 16)`) between `pA` and `pB`
 */
function alignCylinderBetween(mesh, pA, pB, radius = 0.025) {
  const dx = pB.x - pA.x;
  const dy = pB.y - pA.y;
  const dz = pB.z - pA.z;
  const len = Math.max(0.001, Math.hypot(dx, dy, dz));
  mesh.position.set((pA.x + pB.x) * 0.5, (pA.y + pB.y) * 0.5, (pA.z + pB.z) * 0.5);
  mesh.scale.set(radius, len, radius);
  const dir = new THREE.Vector3(dx / len, dy / len, dz / len);
  mesh.quaternion.setFromUnitVectors(new THREE.Vector3(0, 1, 0), dir);
}

/**
 * Samples a point `{ x, y, z, owner }` along a Quinelings SDK ridge (`ridge.line`)
 */
function sampleSdkRidgePoint(ridge, u) {
  const line = ridge?.line || ridge?.points;
  if (!Array.isArray(line) || line.length === 0) return null;
  const idx = Math.min(line.length - 1, Math.floor(clamp(u, 0, 1) * (line.length - 1)));
  const pt = line[idx];
  if (!pt) return null;
  if (Array.isArray(pt)) {
    return { x: pt[0] || 0, y: pt[1] || 0, z: pt[2] || 0, owner: 0 };
  }
  return {
    x: Number.isFinite(pt.x) ? pt.x : 0,
    y: Number.isFinite(pt.y) ? pt.y : 0,
    z: Number.isFinite(pt.z) ? pt.z : 0,
    owner: Number.isInteger(pt.owner) ? pt.owner : null,
  };
}

/**
 * Maps `(u, v)` and SDK manifold Perturbations `(wx, wy, wz)` directly onto the active
 * species' 3D macro-anatomical surface so the SDK caustic ribbons and RGB spores hug
 * the shell, siphonophore wave, coral limbs, moth wings, lotus petals, jellyfish bell,
 * comet shockwave cone, or hollow tokamak torus — zero flat 2D cages!
 */
function embedOnSpeciesSurface(speciesId, u, v, rIdx = 0, sdkDelta = 0) {
  switch (speciesId) {
    case 'factorial': {
      // Conispiral Nautilus Shell surface (`r <= 0.40`)
      const theta = u * 2.05 * TAU;
      const rad = 0.022 * Math.exp(0.215 * theta);
      const tubeR = Math.max(0.014, rad * 0.36);
      const roll = (v * 0.5 + 0.5) * TAU + sdkDelta * 0.4;
      const R = rad + (tubeR + 0.004) * Math.cos(roll);
      return {
        x: Math.cos(theta) * R,
        y: Math.sin(theta) * R * 0.78 + 0.02,
        z: Math.sin(theta) * rad * 0.22 + (tubeR + 0.004) * Math.sin(roll),
      };
    }
    case 'list_fold': {
      // Horizontal undulating 3-Zooid Siphonophore wave (`x in [-0.40, +0.40]`)
      const x = lerp(-0.4, 0.4, u);
      const waveY = Math.sin(u * TAU * 1.5) * 0.075;
      const angle = v * Math.PI + u * TAU * 2.0 + sdkDelta;
      const rad = 0.048 + 0.025 * Math.cos(u * TAU * 3.0);
      return {
        x,
        y: waveY + Math.sin(angle) * rad,
        z: Math.cos(angle) * rad,
      };
    }
    case 'tree_fold': {
      // Helical bark filaments climbing the Y-branching coral limbs
      const branchIdx = rIdx % 3;
      if (branchIdx === 0) {
        // Trunk (`y: -0.34 -> -0.06`) + West limb (`(-0.27, +0.16, +0.05)`)
        const pX = u < 0.45 ? 0 : lerp(0, -0.27, (u - 0.45) / 0.55);
        const pY = u < 0.45 ? lerp(-0.34, -0.06, u / 0.45) : lerp(-0.06, 0.16, (u - 0.45) / 0.55);
        const pZ = u < 0.45 ? 0 : lerp(0, 0.05, (u - 0.45) / 0.55);
        const barkAngle = u * TAU * 2.5 + v * Math.PI;
        return {
          x: pX + Math.cos(barkAngle) * 0.036,
          y: pY,
          z: pZ + Math.sin(barkAngle) * 0.036,
        };
      }
      const isNorthEast = branchIdx === 1;
      const endX = isNorthEast ? 0.32 : 0.3;
      const endY = isNorthEast ? 0.26 : 0.25;
      const endZ = isNorthEast ? -0.14 : 0.16;
      const pX = u < 0.4 ? lerp(0, 0.19, u / 0.4) : lerp(0.19, endX, (u - 0.4) / 0.6);
      const pY = u < 0.4 ? lerp(-0.08, 0.04, u / 0.4) : lerp(0.04, endY, (u - 0.4) / 0.6);
      const pZ = u < 0.4 ? lerp(0, -0.02, u / 0.4) : lerp(-0.02, endZ, (u - 0.4) / 0.6);
      const barkAngle = u * TAU * 2.5 + v * Math.PI;
      return {
        x: pX + Math.cos(barkAngle) * 0.03,
        y: pY,
        z: pZ + Math.sin(barkAngle) * 0.03,
      };
    }
    case 'mutual_even_odd': {
      // Bilateral Figure-8 Lorenz Wing veins on Left (`-1`) and Right (`+1`) lobes
      const side = rIdx % 2 === 0 ? -1 : 1;
      const ang = u * TAU;
      const rad = 0.13 + 0.06 * Math.abs(v);
      const tilt = 0.48 * side;
      return {
        x: side * 0.16 + Math.cos(ang) * rad,
        y: Math.sin(ang) * rad * Math.cos(tilt),
        z: side * 0.04 + Math.sin(ang) * rad * Math.sin(tilt) + sdkDelta * 0.04,
      };
    }
    case 'lexical_let': {
      // Upward-cupped Golden-Angle Lotus whorls around the 3 nested crystal cubes
      const angle = rIdx * PHI_GOLDEN_ANGLE + u * 0.85;
      const ring = rIdx % 3;
      const rad = lerp(0.08, 0.22 + ring * 0.055, u);
      const y = lerp(-0.1, 0.14 + ring * 0.05, u * u);
      return {
        x: Math.cos(angle) * rad,
        y,
        z: Math.sin(angle) * rad,
      };
    }
    case 'checked_underflow': {
      // Medusa hemispherical umbrella meridians + trailing tentacle spirals
      if (rIdx < 6) {
        const phi = (rIdx / 6) * TAU + v * 0.35;
        const theta = u * Math.PI * 0.56;
        const R = 0.312;
        return {
          x: R * Math.sin(theta) * Math.cos(phi),
          y: 0.06 + R * Math.cos(theta),
          z: R * Math.sin(theta) * Math.sin(phi),
        };
      }
      const tIdx = rIdx % 16;
      const baseA = (tIdx / 16) * TAU;
      const rad = (tIdx % 2 === 0 ? 0.24 : 0.16) * (1 - 0.18 * u);
      const twist = baseA + u * 0.9 + sdkDelta * 0.3;
      return {
        x: Math.cos(twist) * rad,
        y: lerp(0.02, -0.38, u),
        z: Math.sin(twist) * rad,
      };
    }
    case 'stack_overflow': {
      // Conical ballistic lance vortex rising toward the `y = +0.34` ceiling barrier
      const y = lerp(-0.24, 0.32, u);
      const rad = lerp(0.09, 0.31, u);
      const ang = u * TAU * 2.2 + (rIdx / SDK_RIBBON_COUNT) * TAU;
      return {
        x: Math.cos(ang) * rad,
        y,
        z: Math.sin(ang) * rad,
      };
    }
    case 'fuel_exhaustion':
    default: {
      // Poloidal (1,3) Tokamak Torus surface (`R = 0.32, r = 0.072`) leaving center 100% open!
      const theta = u * TAU + (rIdx / SDK_RIBBON_COUNT) * TAU;
      const phi = 3.0 * theta + v * Math.PI;
      const R = 0.32;
      const r = 0.074;
      return {
        x: (R + r * Math.cos(phi)) * Math.cos(theta),
        y: (R + r * Math.cos(phi)) * Math.sin(theta),
        z: r * Math.sin(phi),
      };
    }
  }
}

/**
 * Builds a scalloped moth wing ExtrudeGeometry (`Lepidoptera Alternans`)
 */
function createScallopedWingGeometry(isForewing = true) {
  const shape = new THREE.Shape();
  shape.moveTo(0, 0);
  if (isForewing) {
    shape.bezierCurveTo(0.08, 0.16, 0.24, 0.22, 0.34, 0.11);
    shape.bezierCurveTo(0.37, 0.04, 0.31, -0.05, 0.22, -0.08);
    shape.bezierCurveTo(0.14, -0.1, 0.06, -0.05, 0, 0);
  } else {
    shape.bezierCurveTo(0.06, -0.02, 0.22, -0.04, 0.26, -0.14);
    shape.bezierCurveTo(0.24, -0.22, 0.14, -0.26, 0.08, -0.22);
    shape.bezierCurveTo(0.04, -0.16, 0.02, -0.08, 0, 0);
  }
  return new THREE.ExtrudeGeometry(shape, {
    depth: 0.005,
    bevelEnabled: false,
    curveSegments: 18,
  });
}

/**
 * Builds the 3D Procedural QDL Quineling Core Subsystem (`Designer 8`) with
 * 8 unmistakable species sub-groups + surface-embedded Quinelings SDK manifold ribbons & RGB spores.
 */
export function createQuinelingSubsystem() {
  const group = new THREE.Group();
  group.name = 'Designer8_QuinelingCore';

  const coreRoot = new THREE.Group();
  coreRoot.name = 'QuinelingOrganismRoot';
  coreRoot.position.set(0, QUINELING_CORE_Y, 0);
  group.add(coreRoot);

  // Two balanced studio organism lights (Key + Cool Rim) so organic tissue never looks overblown
  const corePointLight = new THREE.PointLight(0xf59e0b, 0.85, 2.4, 1.5);
  corePointLight.position.set(0, 0.06, 0.08);
  coreRoot.add(corePointLight);

  const rimPointLight = new THREE.PointLight(0x9ad7ff, 0.42, 2.4, 1.5);
  rimPointLight.position.set(0.22, 0.18, 0.26);
  coreRoot.add(rimPointLight);

  // ==========================================================================
  // 1. REAL QUINELINGS SDK 3D MANIFOLD & CAUSTIC RIBBON WEB (`WovenBody.frame`)
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
    emissive: new THREE.Color(0x041018),
    emissiveIntensity: 0.75,
    roughness: 0.38,
    metalness: 0.04,
    transparent: true,
    opacity: 0.56,
    side: THREE.DoubleSide,
    depthWrite: false,
    polygonOffset: true,
    polygonOffsetFactor: -1,
    polygonOffsetUnits: -1,
  });
  const sdkManifoldRibbonMesh = new THREE.Mesh(sdkRibbonGeom, sdkRibbonMat);
  sdkManifoldRibbonMesh.name = 'QuinelingSdkManifoldWeb';
  coreRoot.add(sdkManifoldRibbonMesh);

  // ==========================================================================
  // 2. 64 LOSSLESS RGB GENOME SPORES (`Quinelings.encodeColors(quineProgram)`)
  // ==========================================================================
  const sporeGeom = new THREE.OctahedronGeometry(0.012, 0);
  const sporeMat = new THREE.MeshBasicMaterial({
    color: 0xffffff,
    transparent: true,
    opacity: 0.88,
  });
  const sporeMesh = new THREE.InstancedMesh(sporeGeom, sporeMat, SPORE_COUNT);
  sporeMesh.instanceMatrix.setUsage(THREE.DynamicDrawUsage);
  const sporeColorInit = new THREE.Color(0x38bdf8);
  for (let i = 0; i < SPORE_COUNT; i += 1) {
    sporeMesh.setColorAt(i, sporeColorInit);
  }
  if (sporeMesh.instanceColor) {
    sporeMesh.instanceColor.needsUpdate = true;
  }
  coreRoot.add(sporeMesh);

  // ==========================================================================
  // 3. VERTICAL CHAMBER TENDRILS (Floor Rule Modules <-> Quineling <-> 6 Rings)
  // ==========================================================================
  const tendrilGeom = new THREE.BufferGeometry();
  const tendrilPositions = new Float32Array(TENDRIL_VERTEX_COUNT * 3);
  const tendrilColors = new Float32Array(TENDRIL_VERTEX_COUNT * 3);
  const tendrilNormals = new Float32Array(TENDRIL_VERTEX_COUNT * 3);
  tendrilGeom.setAttribute('position', new THREE.BufferAttribute(tendrilPositions, 3));
  tendrilGeom.setAttribute('color', new THREE.BufferAttribute(tendrilColors, 3));
  tendrilGeom.setAttribute('normal', new THREE.BufferAttribute(tendrilNormals, 3));
  tendrilGeom.setIndex(buildRibbonIndices(TENDRIL_CURVE_COUNT, TENDRIL_SEGMENTS));

  const tendrilMat = new THREE.MeshBasicMaterial({
    vertexColors: true,
    transparent: true,
    opacity: 0.34,
    side: THREE.DoubleSide,
    depthWrite: false,
  });
  const chamberTendrilsMesh = new THREE.Mesh(tendrilGeom, tendrilMat);
  coreRoot.add(chamberTendrilsMesh);

  // ==========================================================================
  // 4. EIGHT RADICALLY DISTINCT 3D SPECIES ARCHITECTURES
  // ==========================================================================
  const speciesGroups = {};

  // --------------------------------------------------------------------------
  // SPECIES 1: `factorial` — CONISPIRAL 3D NAUTILUS SHELL TUBE + DYNAMIC SEPTA
  // --------------------------------------------------------------------------
  const nautilusGroup = new THREE.Group();
  nautilusGroup.name = 'Species_Factorial_Nautilus';
  coreRoot.add(nautilusGroup);
  speciesGroups.factorial = nautilusGroup;

  const nautilusShellMat = new THREE.MeshPhysicalMaterial({
    color: 0xfbbf24,
    emissive: new THREE.Color(0xb45309),
    emissiveIntensity: 0.34,
    roughness: 0.22,
    metalness: 0.18,
    clearcoat: 0.65,
    clearcoatRoughness: 0.14,
    iridescence: 0.85,
    iridescenceIOR: 1.45,
    transparent: true,
    opacity: 0.92,
  });

  // Conispiral shell tube (`r <= 0.40`) + 18 nacreous growth rings along the tube
  const nautilusCurvePts = [];
  for (let i = 0; i <= 72; i += 1) {
    const u = i / 72;
    const theta = u * 2.05 * TAU;
    const rad = 0.022 * Math.exp(0.215 * theta);
    nautilusCurvePts.push(
      new THREE.Vector3(
        Math.cos(theta) * rad,
        Math.sin(theta) * rad * 0.78 + 0.02,
        Math.sin(theta) * rad * 0.22
      )
    );
  }
  const nautilusSpline = new THREE.CatmullRomCurve3(nautilusCurvePts);
  const nautilusTubeMesh = new THREE.Mesh(
    new THREE.TubeGeometry(nautilusSpline, 96, 0.028, 16, false),
    nautilusShellMat
  );
  nautilusGroup.add(nautilusTubeMesh);

  const nautilusRibs = [];
  for (let i = 0; i < 20; i += 1) {
    const u = (i + 1) / 20;
    const theta = u * 2.05 * TAU;
    const rad = 0.022 * Math.exp(0.215 * theta);
    const tubeR = Math.max(0.015, rad * 0.38);
    const pt = nautilusSpline.getPointAt(u);
    const tangent = nautilusSpline.getTangentAt(u);
    const ribMat = nautilusShellMat.clone();
    const rib = new THREE.Mesh(new THREE.TorusGeometry(tubeR, tubeR * 0.22, 10, 22), ribMat);
    rib.position.copy(pt);
    rib.quaternion.setFromUnitVectors(new THREE.Vector3(0, 0, 1), tangent);
    nautilusGroup.add(rib);
    nautilusRibs.push({ mesh: rib, mat: ribMat, u, tubeR });
  }

  // Pool of up to 10 internal pearlescent chamber septa discs (`fact(N)..fact(0)`)
  const MAX_NAUTILUS_CHAMBERS = 10;
  const nautilusChambers = [];
  for (let c = 0; c < MAX_NAUTILUS_CHAMBERS; c += 1) {
    const sepMat = new THREE.MeshPhysicalMaterial({
      color: 0x34d399,
      emissive: new THREE.Color(0x059669),
      emissiveIntensity: 0.48,
      roughness: 0.2,
      metalness: 0.08,
      transparent: true,
      opacity: 0.88,
    });
    const sep = new THREE.Mesh(new THREE.CylinderGeometry(1, 0.9, 0.012, 20), sepMat);
    nautilusGroup.add(sep);
    nautilusChambers.push({ mesh: sep, mat: sepMat, index: c });
  }

  // Pearlescent Nautilus Aperture Hood at the outer mouth of the shell
  const nautilusHoodGeom = new THREE.SphereGeometry(0.135, 24, 16, 0, Math.PI * 1.25, 0, Math.PI * 0.62);
  const nautilusHoodMat = new THREE.MeshPhysicalMaterial({
    color: 0xfde68a,
    emissive: new THREE.Color(0xd97706),
    emissiveIntensity: 0.28,
    roughness: 0.22,
    metalness: 0.12,
    clearcoat: 0.6,
    iridescence: 0.65,
    transparent: true,
    opacity: 0.86,
    side: THREE.DoubleSide,
  });
  const nautilusHood = new THREE.Mesh(nautilusHoodGeom, nautilusHoodMat);
  nautilusHood.position.copy(nautilusCurvePts[nautilusCurvePts.length - 1]);
  nautilusHood.rotation.set(0.3, 0.5, -0.8);
  nautilusGroup.add(nautilusHood);

  // --------------------------------------------------------------------------
  // SPECIES 2: `list_fold` — CONTINUOUS SILK SIPHONOPHORE + DYNAMIC ZOOID BELLS
  // --------------------------------------------------------------------------
  const ribbonGroup = new THREE.Group();
  ribbonGroup.name = 'Species_ListFold_Siphonophore';
  coreRoot.add(ribbonGroup);
  speciesGroups.list_fold = ribbonGroup;

  // Continuous undulating silk membrane (`writeRibbonStrip` instead of flat box cards!)
  const SILK_STRANDS = 3;
  const SILK_SEGMENTS = 28;
  const silkGeom = new THREE.BufferGeometry();
  const silkPositions = new Float32Array(SILK_STRANDS * SILK_SEGMENTS * 2 * 3);
  const silkColors = new Float32Array(SILK_STRANDS * SILK_SEGMENTS * 2 * 3);
  const silkNormals = new Float32Array(SILK_STRANDS * SILK_SEGMENTS * 2 * 3);
  silkGeom.setAttribute('position', new THREE.BufferAttribute(silkPositions, 3));
  silkGeom.setAttribute('color', new THREE.BufferAttribute(silkColors, 3));
  silkGeom.setAttribute('normal', new THREE.BufferAttribute(silkNormals, 3));
  silkGeom.setIndex(buildRibbonIndices(SILK_STRANDS, SILK_SEGMENTS));
  const silkMat = new THREE.MeshPhysicalMaterial({
    color: 0xffffff,
    vertexColors: true,
    emissive: new THREE.Color(0x0891b2),
    emissiveIntensity: 0.42,
    roughness: 0.28,
    metalness: 0.05,
    transparent: true,
    opacity: 0.84,
    side: THREE.DoubleSide,
  });
  const silkRibbonMesh = new THREE.Mesh(silkGeom, silkMat);
  ribbonGroup.add(silkRibbonMesh);

  // Pool of up to 6 glowing `cons` Zooid Bells (`[5, 8, 3]` or user-edited list)
  const MAX_ZOOID_PODS = 6;
  const listZooidPods = [];
  for (let zIdx = 0; zIdx < MAX_ZOOID_PODS; zIdx += 1) {
    const podGroup = new THREE.Group();
    ribbonGroup.add(podGroup);

    const bellMat = new THREE.MeshPhysicalMaterial({
      color: zIdx % 2 === 1 ? 0x22d3ee : 0x38bdf8,
      emissive: new THREE.Color(zIdx % 2 === 1 ? 0x0891b2 : 0x7c3aed),
      emissiveIntensity: 0.48,
      roughness: 0.18,
      metalness: 0.05,
      clearcoat: 0.5,
      iridescence: 0.6,
      transparent: true,
      opacity: 0.88,
    });
    const bell = new THREE.Mesh(new THREE.SphereGeometry(1, 24, 18), bellMat);
    podGroup.add(bell);

    const ringMat = new THREE.MeshPhysicalMaterial({
      color: 0xc084fc,
      emissive: new THREE.Color(0x9333ea),
      emissiveIntensity: 0.52,
      roughness: 0.25,
      metalness: 0.12,
    });
    const ring = new THREE.Mesh(new THREE.TorusGeometry(1.15, 0.11, 10, 28), ringMat);
    ring.rotation.y = Math.PI * 0.5;
    podGroup.add(ring);

    listZooidPods.push({ group: podGroup, bell, bellMat, ring, ringMat, baseX: 0, baseY: 0, radius: 0.08 });
  }

  // --------------------------------------------------------------------------
  // SPECIES 3: `tree_fold` — ASYMMETRIC Y-BRANCHING BINARY CORAL TREE (EMERALD/GOLD)
  // --------------------------------------------------------------------------
  const coralGroup = new THREE.Group();
  coralGroup.name = 'Species_TreeFold_CoralTree';
  coreRoot.add(coralGroup);
  speciesGroups.tree_fold = coralGroup;

  const coralBranches = [];
  const coralPolypCrowns = [];
  const cylUnitGeom = new THREE.CylinderGeometry(0.82, 1.0, 1.0, 16);
  const treeBranchDefs = [
    { pA: new THREE.Vector3(0, -0.34, 0.02), pB: new THREE.Vector3(0, -0.07, -0.02), r: 0.042, side: 'root', color: 0x10b981 },
    { pA: new THREE.Vector3(0, -0.07, -0.02), pB: new THREE.Vector3(-0.26, 0.15, 0.07), r: 0.031, side: 'west', color: 0x34d399 },
    { pA: new THREE.Vector3(0, -0.07, -0.02), pB: new THREE.Vector3(0.19, 0.04, -0.05), r: 0.032, side: 'east', color: 0xf59e0b },
    { pA: new THREE.Vector3(0.19, 0.04, -0.05), pB: new THREE.Vector3(0.32, 0.26, -0.15), r: 0.024, side: 'east_left', color: 0xfbbf24 },
    { pA: new THREE.Vector3(0.19, 0.04, -0.05), pB: new THREE.Vector3(0.3, 0.25, 0.16), r: 0.025, side: 'east_right', color: 0x2dd4bf },
    { pA: new THREE.Vector3(-0.14, 0.04, 0.03), pB: new THREE.Vector3(-0.17, 0.27, -0.12), r: 0.019, side: 'west', color: 0x34d399 },
  ];
  for (const def of treeBranchDefs) {
    const bMat = new THREE.MeshPhysicalMaterial({
      color: def.color,
      emissive: new THREE.Color(def.color).multiplyScalar(0.42),
      emissiveIntensity: 0.42,
      roughness: 0.36,
      metalness: 0.06,
      clearcoat: 0.35,
    });
    const bMesh = new THREE.Mesh(cylUnitGeom, bMat);
    alignCylinderBetween(bMesh, def.pA, def.pB, def.r);
    coralGroup.add(bMesh);
    coralBranches.push({ mesh: bMesh, mat: bMat, def });
  }

  // Rounded Coral Polyp Rosette Crowns at the leaf tips
  const leafCrownDefs = [
    { pos: new THREE.Vector3(-0.26, 0.15, 0.07), side: 'west', color: 0x34d399 },
    { pos: new THREE.Vector3(0.32, 0.26, -0.15), side: 'east_left', color: 0xfbbf24 },
    { pos: new THREE.Vector3(0.3, 0.25, 0.16), side: 'east_right', color: 0x2dd4bf },
    { pos: new THREE.Vector3(-0.17, 0.27, -0.12), side: 'west', color: 0x10b981 },
  ];
  const polypPetalGeom = new THREE.SphereGeometry(0.024, 12, 10);
  polypPetalGeom.scale(1.45, 0.52, 0.78);
  for (const lc of leafCrownDefs) {
    const crownSub = new THREE.Group();
    crownSub.position.copy(lc.pos);
    const cMat = new THREE.MeshPhysicalMaterial({
      color: lc.color,
      emissive: new THREE.Color(lc.color),
      emissiveIntensity: 0.48,
      roughness: 0.28,
      metalness: 0.05,
      clearcoat: 0.45,
    });
    const coreBulb = new THREE.Mesh(new THREE.SphereGeometry(0.044, 16, 12), cMat);
    crownSub.add(coreBulb);
    for (let p = 0; p < 6; p += 1) {
      const a = (p / 6) * TAU;
      const petal = new THREE.Mesh(polypPetalGeom, cMat);
      petal.position.set(Math.cos(a) * 0.044, 0.018, Math.sin(a) * 0.044);
      petal.rotation.y = -a;
      petal.rotation.z = 0.38;
      crownSub.add(petal);
    }
    coralGroup.add(crownSub);
    coralPolypCrowns.push({ group: crownSub, mat: cMat, side: lc.side });
  }

  // --------------------------------------------------------------------------
  // SPECIES 4: `mutual_even_odd` — SCALLOPED LUNA MOTH + RK4 LORENZ STRANGE ATTRACTOR
  // --------------------------------------------------------------------------
  const mothGroup = new THREE.Group();
  mothGroup.name = 'Species_MutualEvenOdd_LunaMoth';
  coreRoot.add(mothGroup);
  speciesGroups.mutual_even_odd = mothGroup;

  const thoraxMat = new THREE.MeshPhysicalMaterial({
    color: 0xf8fafc,
    emissive: new THREE.Color(0x6366f1),
    emissiveIntensity: 0.38,
    roughness: 0.32,
    metalness: 0.08,
  });
  const thoraxMesh = new THREE.Mesh(new THREE.CapsuleGeometry(0.028, 0.24, 8, 16), thoraxMat);
  mothGroup.add(thoraxMesh);

  const leftWingPivot = new THREE.Group();
  leftWingPivot.position.set(-0.022, 0.02, 0);
  mothGroup.add(leftWingPivot);

  const rightWingPivot = new THREE.Group();
  rightWingPivot.position.set(0.022, 0.02, 0);
  mothGroup.add(rightWingPivot);

  const leftWingMat = new THREE.MeshPhysicalMaterial({
    color: 0x06b6d4,
    emissive: new THREE.Color(0x0891b2),
    emissiveIntensity: 0.48,
    roughness: 0.28,
    metalness: 0.06,
    iridescence: 0.9,
    clearcoat: 0.45,
    transparent: true,
    opacity: 0.88,
    side: THREE.DoubleSide,
  });
  const rightWingMat = new THREE.MeshPhysicalMaterial({
    color: 0xa855f7,
    emissive: new THREE.Color(0x7e22ce),
    emissiveIntensity: 0.48,
    roughness: 0.28,
    metalness: 0.06,
    iridescence: 0.9,
    clearcoat: 0.45,
    transparent: true,
    opacity: 0.88,
    side: THREE.DoubleSide,
  });

  const foreWingExtrude = createScallopedWingGeometry(true);
  const hindWingExtrude = createScallopedWingGeometry(false);

  const rightFore = new THREE.Mesh(foreWingExtrude, rightWingMat);
  const rightHind = new THREE.Mesh(hindWingExtrude, rightWingMat);
  rightWingPivot.add(rightFore, rightHind);

  const leftFore = new THREE.Mesh(foreWingExtrude, leftWingMat);
  leftFore.scale.x = -1;
  const leftHind = new THREE.Mesh(hindWingExtrude, leftWingMat);
  leftHind.scale.x = -1;
  leftWingPivot.add(leftFore, leftHind);

  // True 3D Figure-8 RK4 Lorenz Strange Attractor Tube around the Moth
  const lorenzPts = [];
  let lx = 0.18;
  let ly = 0.24;
  let lz = 0.12;
  const dtL = 0.0055;
  for (let i = 0; i < 620; i += 1) {
    const dx = 10 * (ly - lx);
    const dy = lx * (28 - lz) - ly;
    const dz = lx * ly - (8 / 3) * lz;
    lx += dx * dtL;
    ly += dy * dtL;
    lz += dz * dtL;
    if (i >= 180 && i % 2 === 0) {
      lorenzPts.push(new THREE.Vector3(lx * 0.017, (lz - 25) * 0.012, ly * 0.009));
    }
  }
  const lorenzTubeMat = new THREE.MeshPhysicalMaterial({
    color: 0xe0e7ff,
    emissive: new THREE.Color(0x818cf8),
    emissiveIntensity: 0.65,
    roughness: 0.2,
    metalness: 0.1,
  });
  const lorenzOrbitMesh = new THREE.Mesh(
    new THREE.TubeGeometry(new THREE.CatmullRomCurve3(lorenzPts), 180, 0.006, 8, false),
    lorenzTubeMat
  );
  mothGroup.add(lorenzOrbitMesh);

  // --------------------------------------------------------------------------
  // SPECIES 5: `lexical_let` — 3 NESTED CRYSTAL CUBES IN UPWARD-BLOOMING LOTUS
  // --------------------------------------------------------------------------
  const seedLotusGroup = new THREE.Group();
  seedLotusGroup.name = 'Species_LexicalLet_CubeLotus';
  coreRoot.add(seedLotusGroup);
  speciesGroups.lexical_let = seedLotusGroup;

  const nestedCubes = [];
  const cubeSizes = [0.11, 0.2, 0.29]; // seed=12, sq=144, cb=1728
  const cubeColors = [0x38bdf8, 0xa855f7, 0xfbbf24];
  for (let c = 0; c < 3; c += 1) {
    const cSub = new THREE.Group();
    cSub.rotation.set(Math.PI * 0.25, Math.PI * 0.25, 0);
    const boxGeom = new THREE.BoxGeometry(cubeSizes[c], cubeSizes[c], cubeSizes[c]);
    const cMat = new THREE.MeshPhysicalMaterial({
      color: cubeColors[c],
      emissive: new THREE.Color(cubeColors[c]),
      emissiveIntensity: 0.34,
      roughness: 0.15,
      metalness: 0.08,
      clearcoat: 0.65,
      iridescence: 0.6,
      transparent: true,
      opacity: c === 0 ? 0.86 : c === 1 ? 0.32 : 0.18,
      depthWrite: c === 0,
    });
    const cMesh = new THREE.Mesh(boxGeom, cMat);
    cSub.add(cMesh);
    seedLotusGroup.add(cSub);
    nestedCubes.push({ group: cSub, mesh: cMesh, mat: cMat, index: c });
  }

  // 24 Upward-Blooming Golden-Angle (`137.5°`) Lotus Petals (`rotation.order = 'YXZ'`, tips up!)
  const lotusPetals = [];
  const petalConeGeom = new THREE.ConeGeometry(0.044, 0.18, 6);
  petalConeGeom.scale(1.3, 1.0, 0.32);
  for (let p = 0; p < 24; p += 1) {
    const angle = p * PHI_GOLDEN_ANGLE;
    const ring = Math.floor(p / 8); // 3 whorls of 8 petals
    const r = 0.11 + ring * 0.068;
    const pMat = new THREE.MeshPhysicalMaterial({
      color: cubeColors[ring],
      emissive: new THREE.Color(cubeColors[ring]),
      emissiveIntensity: 0.35,
      roughness: 0.28,
      metalness: 0.06,
      clearcoat: 0.4,
      transparent: true,
      opacity: 0.88,
    });
    const pMesh = new THREE.Mesh(petalConeGeom, pMat);
    pMesh.position.set(Math.cos(angle) * r, -0.05 + ring * 0.032, Math.sin(angle) * r);
    pMesh.rotation.order = 'YXZ';
    pMesh.rotation.y = -angle;
    pMesh.rotation.z = -0.58 - ring * 0.16; // Tips point UPWARD and outward like a true lotus chalice!
    seedLotusGroup.add(pMesh);
    lotusPetals.push({ mesh: pMesh, mat: pMat, ring, angle, r });
  }

  // --------------------------------------------------------------------------
  // SPECIES 6: `checked_underflow` — MEDUSA JELLYFISH BELL + 16 SINUOUS TENTACLES
  // --------------------------------------------------------------------------
  const jellyGroup = new THREE.Group();
  jellyGroup.name = 'Species_CheckedUnderflow_MedusaJelly';
  coreRoot.add(jellyGroup);
  speciesGroups.checked_underflow = jellyGroup;

  const jellyBellMat = new THREE.MeshPhysicalMaterial({
    color: 0xf43f5e,
    emissive: new THREE.Color(0xbe123c),
    emissiveIntensity: 0.42,
    roughness: 0.22,
    metalness: 0.0,
    clearcoat: 0.65,
    iridescence: 0.72,
    transparent: true,
    opacity: 0.82,
    side: THREE.DoubleSide,
  });
  const jellyBellGeom = new THREE.SphereGeometry(0.3, 32, 18, 0, TAU, 0, Math.PI * 0.58);
  const jellyBellMesh = new THREE.Mesh(jellyBellGeom, jellyBellMat);
  jellyBellMesh.position.set(0, 0.06, 0);
  jellyGroup.add(jellyBellMesh);

  const jellyRimMat = new THREE.MeshPhysicalMaterial({
    color: 0xfbbf24,
    emissive: new THREE.Color(0xd97706),
    emissiveIntensity: 0.48,
    roughness: 0.28,
    metalness: 0.08,
  });
  const jellyRimMesh = new THREE.Mesh(new THREE.TorusGeometry(0.285, 0.013, 12, 32), jellyRimMat);
  jellyRimMesh.position.set(0, 0.02, 0);
  jellyRimMesh.rotation.x = Math.PI * 0.5;
  jellyGroup.add(jellyRimMesh);

  // 16 Sinuous Curved UInt128 Byte-Limb Tentacles (`writeRibbonStrip` spline ribbons)
  const JELLY_TENTACLE_COUNT = 16;
  const JELLY_TENTACLE_SEGMENTS = 10;
  const jellyTentacleGeom = new THREE.BufferGeometry();
  const jellyTentaclePos = new Float32Array(JELLY_TENTACLE_COUNT * JELLY_TENTACLE_SEGMENTS * 2 * 3);
  const jellyTentacleCol = new Float32Array(JELLY_TENTACLE_COUNT * JELLY_TENTACLE_SEGMENTS * 2 * 3);
  const jellyTentacleNorm = new Float32Array(JELLY_TENTACLE_COUNT * JELLY_TENTACLE_SEGMENTS * 2 * 3);
  jellyTentacleGeom.setAttribute('position', new THREE.BufferAttribute(jellyTentaclePos, 3));
  jellyTentacleGeom.setAttribute('color', new THREE.BufferAttribute(jellyTentacleCol, 3));
  jellyTentacleGeom.setAttribute('normal', new THREE.BufferAttribute(jellyTentacleNorm, 3));
  jellyTentacleGeom.setIndex(buildRibbonIndices(JELLY_TENTACLE_COUNT, JELLY_TENTACLE_SEGMENTS));
  const jellyTentacleMat = new THREE.MeshPhysicalMaterial({
    color: 0xffffff,
    vertexColors: true,
    emissive: new THREE.Color(0xbe123c),
    emissiveIntensity: 0.55,
    roughness: 0.28,
    metalness: 0.05,
    side: THREE.DoubleSide,
  });
  const jellyTentaclesMesh = new THREE.Mesh(jellyTentacleGeom, jellyTentacleMat);
  jellyGroup.add(jellyTentaclesMesh);

  // --------------------------------------------------------------------------
  // SPECIES 7: `stack_overflow` — BALLISTIC COMET + DYNAMIC PANCAKED CEILING RINGS
  // --------------------------------------------------------------------------
  const cometGroup = new THREE.Group();
  cometGroup.name = 'Species_StackOverflow_CometHorizon';
  coreRoot.add(cometGroup);
  speciesGroups.stack_overflow = cometGroup;

  const barrierMat = new THREE.MeshPhysicalMaterial({
    color: 0xe11d48,
    emissive: new THREE.Color(0xbe123c),
    emissiveIntensity: 0.58,
    roughness: 0.22,
    metalness: 0.12,
    transparent: true,
    opacity: 0.52,
    side: THREE.DoubleSide,
  });
  const barrierDisc = new THREE.Mesh(new THREE.CylinderGeometry(0.35, 0.35, 0.012, 36), barrierMat);
  barrierDisc.position.set(0, 0.34, 0);
  cometGroup.add(barrierDisc);

  const barrierRim = new THREE.Mesh(
    new THREE.TorusGeometry(0.35, 0.012, 12, 40),
    new THREE.MeshBasicMaterial({ color: 0xf43f5e })
  );
  barrierRim.position.set(0, 0.34, 0);
  barrierRim.rotation.x = Math.PI * 0.5;
  cometGroup.add(barrierRim);

  const cometLanceSub = new THREE.Group();
  cometLanceSub.rotation.z = -0.32;
  cometGroup.add(cometLanceSub);

  const cometHeadMat = new THREE.MeshPhysicalMaterial({
    color: 0xfef08a,
    emissive: new THREE.Color(0xf97316),
    emissiveIntensity: 0.78,
    roughness: 0.16,
    metalness: 0.1,
  });
  const cometHeadMesh = new THREE.Mesh(new THREE.OctahedronGeometry(0.072, 1), cometHeadMat);
  cometHeadMesh.position.set(0, 0.22, 0);
  cometLanceSub.add(cometHeadMesh);

  const cometTailMat = new THREE.MeshPhysicalMaterial({
    color: 0xf97316,
    emissive: new THREE.Color(0xe11d48),
    emissiveIntensity: 0.48,
    roughness: 0.28,
    metalness: 0.06,
    transparent: true,
    opacity: 0.78,
  });
  // Closed tapered cone (`openEnded = false`) so the tail never looks like a hollow sheet!
  const cometTailMesh = new THREE.Mesh(new THREE.ConeGeometry(0.125, 0.52, 20, 1, false), cometTailMat);
  cometTailMesh.position.set(0, -0.05, 0);
  cometLanceSub.add(cometTailMesh);

  // Pool of up to 10 Horizontal Julia-Set Shockwave Rings (`stackLimit` driven)
  const MAX_COMET_RINGS = 10;
  const cometShockRings = [];
  for (let rIdx = 0; rIdx < MAX_COMET_RINGS; rIdx += 1) {
    const rMat = new THREE.MeshPhysicalMaterial({
      color: 0xfb923c,
      emissive: new THREE.Color(0xea580c),
      emissiveIntensity: 0.5,
      roughness: 0.24,
      metalness: 0.12,
    });
    const rMesh = new THREE.Mesh(new THREE.TorusGeometry(0.22, 0.013, 10, 32), rMat);
    rMesh.rotation.x = Math.PI * 0.5;
    cometGroup.add(rMesh);
    cometShockRings.push({ mesh: rMesh, mat: rMat, index: rIdx });
  }

  // --------------------------------------------------------------------------
  // SPECIES 8: `fuel_exhaustion` — HOLLOW OPEN-CENTER 14-SEGMENT TOKAMAK TORUS
  // --------------------------------------------------------------------------
  const torusGroup = new THREE.Group();
  torusGroup.name = 'Species_FuelExhaustion_HollowTorus';
  torusGroup.rotation.x = 0.28; // Tilted so the open center hole is unmistakable and stays well above the floor
  torusGroup.rotation.z = 0.16;
  coreRoot.add(torusGroup);
  speciesGroups.fuel_exhaustion = torusGroup;

  const torusFuelCells = [];
  const cellArcGeom = new THREE.TorusGeometry(0.32, 0.066, 16, 12, (TAU / 14) * 0.82);
  for (let fIdx = 0; fIdx < 14; fIdx += 1) {
    const angle = (fIdx / 14) * TAU;
    const cMat = new THREE.MeshPhysicalMaterial({
      color: 0x38bdf8,
      emissive: new THREE.Color(0x0284c7),
      emissiveIntensity: 0.52,
      roughness: 0.22,
      metalness: 0.28,
      clearcoat: 0.65,
      transparent: true,
      opacity: 0.92,
    });
    const cellMesh = new THREE.Mesh(cellArcGeom, cMat);
    cellMesh.rotation.z = angle;
    torusGroup.add(cellMesh);
    torusFuelCells.push({ mesh: cellMesh, mat: cMat, index: fIdx, angle });
  }

  // True Poloidal (1,3) Winding Tube hugging the `R = 0.32, r = 0.070` torus surface (`r <= 0.39`)
  const braidPts = [];
  for (let i = 0; i <= 180; i += 1) {
    const a = (i / 180) * TAU;
    const b = 3.0 * a;
    const R = 0.32 + 0.072 * Math.cos(b);
    braidPts.push(new THREE.Vector3(R * Math.cos(a), R * Math.sin(a), 0.072 * Math.sin(b)));
  }
  const torusBraidMat = new THREE.MeshPhysicalMaterial({
    color: 0xfbbf24,
    emissive: new THREE.Color(0xd97706),
    emissiveIntensity: 0.48,
    roughness: 0.24,
    metalness: 0.35,
  });
  const torusBraidMesh = new THREE.Mesh(
    new THREE.TubeGeometry(new THREE.CatmullRomCurve3(braidPts, true), 220, 0.0065, 8, true),
    torusBraidMat
  );
  torusGroup.add(torusBraidMesh);

  function refreshInspectMetadata(artifact, stepState) {
    const species = artifact.speciesMeta;
    const tel = getQuinelingTelemetry(stepState);
    const meta = {
      designer: 'D8 · QDL Quineling Core Organism',
      title: `${species.speciesName} (${species.commonTitle})`,
      Check: `${species.family} · ${species.mechanism} · ${tel.morphGesture}`,
      body: `${species.description} Compiled from Sherlock AST (${artifact.graph.nodes.length} QDL task nodes, ${artifact.quineBytes}B self-reproducing Quine verified=${artifact.quineVerified}).`,
    };
    tagInspectable(coreRoot, meta);
    tagInspectable(sdkManifoldRibbonMesh, meta);
  }

  const _dummy = new THREE.Object3D();
  const _color = new THREE.Color();
  const _sdkPts = Array.from({ length: SDK_RIBBON_SEGMENTS }, () => new THREE.Vector3());
  const _silkPts = Array.from({ length: SILK_SEGMENTS }, () => new THREE.Vector3());
  const _tentaclePts = Array.from({ length: JELLY_TENTACLE_SEGMENTS }, () => new THREE.Vector3());
  const _tendrilPts = Array.from({ length: TENDRIL_SEGMENTS }, () => new THREE.Vector3());

  let lastCacheKey = '';
  let lastStepRidgeKey = '';
  let cachedSdkRidges = [];
  let lastSpeciesParent = null;
  let dampedScale = 1.0;
  let dampedTendrilOpacity = 0.34;
  let dampedYOffset = 0.0;

  // ==========================================================================
  // 5. PER-FRAME LIVE EXECUTION MORPHOGENESIS UPDATE (`update(stepState)`)
  // ==========================================================================
  function update(stepState = {}) {
    const ambient = stepState.ambient ?? 0;
    const dt = clamp(stepState.dt ?? 0.016, 0.001, 0.05);
    const step = stepState.step ?? null;
    const stepIndex = stepState.stepIndex ?? step?.stepIndex ?? 0;
    const stepFraction = clamp(stepState.stepFraction ?? 0, 0, 1);
    const q = quinticStep(stepFraction);

    // 1. Compile or fetch cached Quineling artifact from the actual Sherlock program source & trace!
    const artifact = resolveCompiledQuineling(stepState);
    const species = artifact.speciesMeta;
    const speciesId = species.id;
    const entryMorph = artifact.analysis?.entryMorph || {};

    // Toggle visibility so ONLY the active species' 3D architecture is shown!
    for (const [key, sGroup] of Object.entries(speciesGroups)) {
      sGroup.visible = key === speciesId;
    }

    if (artifact.cacheKey !== lastCacheKey) {
      lastCacheKey = artifact.cacheKey;
      refreshInspectMetadata(artifact, stepState);
    }

    // Camera-Lens-Aware Study Scaling (`interior_floor` and `interior_up` keep floor & rings 100% clear!)
    const activePreset = stepState.activePreset || stepState.cameraPreset || 'overview';
    const isFloorStudy = activePreset === 'interior_floor';
    const isUpStudy = activePreset === 'interior_up';
    const targetScale = isFloorStudy ? 0.58 : isUpStudy ? 0.52 : 1.0;
    const targetTendrilOp = isFloorStudy ? 0.08 : isUpStudy ? 0.06 : 0.32;
    const targetYOffset = isUpStudy ? -0.2 : 0.0;
    const dampMix = 1.0 - Math.exp(-8.0 * dt);
    dampedScale = lerp(dampedScale, targetScale, dampMix);
    dampedTendrilOpacity = lerp(dampedTendrilOpacity, targetTendrilOp, dampMix);
    dampedYOffset = lerp(dampedYOffset, targetYOffset, dampMix);

    coreRoot.scale.setScalar(dampedScale);
    tendrilMat.opacity = dampedTendrilOpacity;

    const ruleLine = step?.ruleLine ?? 80;
    const ruleId = step?.ruleId || `A${String(ruleLine).padStart(3, '0')}`;
    const frames = step?.after?.frames ?? [];
    const frameCount = frames.length;
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
    const isAluPrim = ruleLine === 223 || ruleLine === 225;

    // Gentle levitation & breathing inside the hollow chamber
    const breathFreq = 1.3 + liveFuelRatio * 1.1;
    const breathPhase = ambient * breathFreq;
    coreRoot.position.y = QUINELING_CORE_Y + dampedYOffset + Math.sin(breathPhase) * 0.022;
    coreRoot.rotation.y = ambient * 0.22 + (isInPlaceRewrite ? Math.sin(q * Math.PI) * 0.18 : 0);

    corePointLight.color.setHex(
      isFault ? ROLES_1_HEX.faultCrimson : isDone ? ROLES_1_HEX.reportGreen : species.primaryHex
    );
    corePointLight.intensity = isDone ? 1.15 : isFault ? 1.25 : isExhausted ? 0.35 : 0.8 + callDepth * 0.08;

    // ========================================================================
    // 2. SAMPLE REAL QUINELINGS SDK 3D MANIFOLD (`WovenBody.frame` + `Morphology`)
    // ========================================================================
    const stepRidgeKey = `${artifact.cacheKey}|step:${stepIndex}`;
    if (stepRidgeKey !== lastStepRidgeKey) {
      lastStepRidgeKey = stepRidgeKey;
      const stepPhaseRad = (stepIndex * 0.08) * TAU;
      const wf = WovenBody.frame(artifact.wovenBody, stepPhaseRad, {
        budget: 256,
        crests: true,
      });
      if (wf.ridges?.length) {
        cachedSdkRidges = wf.ridges;
      } else {
        const mf = Morphology.surfaceFrame(artifact.shape, stepPhaseRad, true);
        cachedSdkRidges = mf.ridges || [];
      }
    }
    const ridges = cachedSdkRidges;
    const nodeColors = artifact.nodeColors || [];

    const activeExprId =
      step?.after?.control?.exprId ??
      step?.before?.control?.exprId ??
      null;
    const activeNodeId =
      activeExprId !== null && artifact.exprToNodeId?.has(activeExprId)
        ? artifact.exprToNodeId.get(activeExprId)
        : null;
    const activeNodeIdx = activeNodeId
      ? artifact.graph.nodes.findIndex((n) => n.id === activeNodeId)
      : stepIndex % Math.max(1, artifact.graph.nodes.length);

    for (let rIdx = 0; rIdx < SDK_RIBBON_COUNT; rIdx += 1) {
      const ridgeIndex = Math.floor((rIdx * Math.max(1, ridges.length)) / SDK_RIBBON_COUNT);
      const ridge = ridges[ridgeIndex];
      const vCoord = ((rIdx % 8) / 7) * 2 - 1;
      const midSample = sampleSdkRidgePoint(ridge, 0.5);
      const ownerIdx =
        midSample?.owner !== null && midSample?.owner !== undefined && midSample.owner >= 0
          ? midSample.owner
          : rIdx % Math.max(1, nodeColors.length);
      const cSpec = nodeColors[ownerIdx % Math.max(1, nodeColors.length)];
      const rgb = cSpec?.rgb || [56, 189, 248];
      let cr = rgb[0] / 255;
      let cg = rgb[1] / 255;
      let cb = rgb[2] / 255;

      if (ownerIdx === activeNodeIdx) {
        cr = lerp(cr, 1.0, 0.45);
        cg = lerp(cg, 0.92, 0.45);
        cb = lerp(cb, 0.55, 0.45);
      }
      if (isFault) {
        cr = lerp(cr, 0.95, 0.7);
        cg *= 0.25;
        cb *= 0.35;
      } else if (isExhausted) {
        const gray = (cr + cg + cb) / 3;
        cr = lerp(cr, gray * 0.55, 0.75);
        cg = lerp(cg, gray * 0.6, 0.75);
        cb = lerp(cb, gray * 0.68, 0.75);
      }

      for (let s = 0; s < SDK_RIBBON_SEGMENTS; s += 1) {
        const u = s / (SDK_RIBBON_SEGMENTS - 1);
        const rawPt = sampleSdkRidgePoint(ridge, u);
        const sdkWave = rawPt
          ? clamp((rawPt.x + rawPt.y + rawPt.z) * 0.15 + Math.sin(ambient * 1.6 + u * TAU) * 0.04, -0.35, 0.35)
          : Math.sin(ambient * 1.6 + u * TAU) * 0.04;
        const pos = embedOnSpeciesSurface(speciesId, u, vCoord, rIdx, sdkWave);
        _sdkPts[s].set(pos.x, pos.y, pos.z);
      }
      writeRibbonStrip(
        sdkRibbonPositions,
        sdkRibbonColors,
        sdkRibbonNormals,
        rIdx,
        SDK_RIBBON_SEGMENTS,
        _sdkPts,
        ownerIdx === activeNodeIdx ? 0.0055 : 0.0032,
        cr,
        cg,
        cb
      );
    }
    sdkRibbonGeom.attributes.position.needsUpdate = true;
    sdkRibbonGeom.attributes.color.needsUpdate = true;
    sdkRibbonGeom.attributes.normal.needsUpdate = true;

    // ========================================================================
    // 3. ANIMATE THE ACTIVE SPECIES' DISTINCT 3D MACRO-ANATOMY
    // ========================================================================
    if (speciesId === 'factorial') {
      // COILED 3D NAUTILUS SHELL: Dynamic `N + 1` chamber septa (`fact(4)` = 5, `fact(5)` = 6)
      const chamberCount = clamp((entryMorph.entryN ?? 4) + 1, 2, MAX_NAUTILUS_CHAMBERS);
      nautilusGroup.rotation.y = Math.sin(ambient * 0.6) * 0.22;
      for (let i = 0; i < nautilusRibs.length; i += 1) {
        const item = nautilusRibs[i];
        const unlocked = item.u <= clamp((callDepth + 1) / chamberCount, 0.35, 1.0) || isDone;
        const pulse = unlocked ? 1.0 + 0.06 * Math.sin(ambient * 3.0 - item.u * 6.0) : 0.82;
        item.mesh.scale.setScalar(pulse);
        item.mat.color.setHex(isDone ? 0x34d399 : unlocked ? 0xfbbf24 : 0x78350f);
        item.mat.emissiveIntensity = unlocked ? 0.42 : 0.14;
      }
      for (let c = 0; c < MAX_NAUTILUS_CHAMBERS; c += 1) {
        const ch = nautilusChambers[c];
        if (c >= chamberCount) {
          ch.mesh.visible = false;
          continue;
        }
        ch.mesh.visible = true;
        const u = 0.22 + (c / Math.max(1, chamberCount - 1)) * 0.72;
        const theta = u * 2.05 * TAU;
        const rad = 0.022 * Math.exp(0.215 * theta);
        const tubeR = Math.max(0.016, rad * 0.36);
        const pt = nautilusSpline.getPointAt(u);
        const tangent = nautilusSpline.getTangentAt(u);
        ch.mesh.position.copy(pt);
        ch.mesh.quaternion.setFromUnitVectors(new THREE.Vector3(0, 1, 0), tangent);
        const activeChamber = c < callDepth || isDone;
        const lockedGold = c < frameCount && (ruleLine === 149 || controlKindIsReturn(step));
        ch.mat.color.setHex(isDone ? 0x2dd4bf : lockedGold ? 0xfde047 : activeChamber ? 0xf59e0b : 0x475569);
        ch.mat.emissiveIntensity = activeChamber ? 0.68 : 0.18;
        ch.mesh.scale.set(tubeR, 1.0, tubeR);
      }
    } else if (speciesId === 'list_fold') {
      // CONTINUOUS SILK SIPHONOPHORE RIBBON + DYNAMIC `cons` ZOOID BELLS
      const listVals = entryMorph.listValues?.length ? entryMorph.listValues : [5, 8, 3];
      const podCount = clamp(listVals.length, 1, MAX_ZOOID_PODS);
      const activePodIdx = clamp(callDepth - 1, 0, podCount - 1);

      // Update continuous undulating silk membrane
      for (let st = 0; st < SILK_STRANDS; st += 1) {
        const zOff = (st - 1) * 0.032;
        for (let s = 0; s < SILK_SEGMENTS; s += 1) {
          const u = s / (SILK_SEGMENTS - 1);
          const x = lerp(-0.4, 0.4, u);
          const y = Math.sin(u * Math.PI * podCount - ambient * 2.6 + st * 0.4) * 0.072;
          const z = zOff + Math.cos(u * TAU * 1.5 - ambient * 2.0) * 0.045;
          _silkPts[s].set(x, y, z);
        }
        _color.setHex(isDone ? 0x2dd4bf : st === 1 ? 0x22d3ee : 0xa855f7);
        writeRibbonStrip(
          silkPositions,
          silkColors,
          silkNormals,
          st,
          SILK_SEGMENTS,
          _silkPts,
          0.034,
          _color.r,
          _color.g,
          _color.b
        );
      }
      silkGeom.attributes.position.needsUpdate = true;
      silkGeom.attributes.color.needsUpdate = true;
      silkGeom.attributes.normal.needsUpdate = true;

      for (let zIdx = 0; zIdx < MAX_ZOOID_PODS; zIdx += 1) {
        const pod = listZooidPods[zIdx];
        if (zIdx >= podCount) {
          pod.group.visible = false;
          continue;
        }
        pod.group.visible = true;
        const u = podCount === 1 ? 0.5 : 0.15 + (zIdx / (podCount - 1)) * 0.7;
        const val = listVals[zIdx] ?? 5;
        const bellR = clamp(0.052 + val * 0.0045, 0.052, 0.105);
        const x = lerp(-0.4, 0.4, u);
        const y = Math.sin(u * Math.PI * podCount - ambient * 2.6) * 0.072;
        const z = Math.cos(u * TAU * 1.5 - ambient * 2.0) * 0.045;
        pod.group.position.set(x, y, z);
        const isCurrent = zIdx === activePodIdx || isDone;
        const pulse = isCurrent ? 1.12 + 0.06 * Math.sin(ambient * 4.0) : 0.94;
        pod.bell.scale.set(bellR * 1.22 * pulse, bellR * 0.92 * pulse, bellR * 0.92 * pulse);
        pod.ring.scale.setScalar(bellR * pulse);
        pod.bellMat.color.setHex(isDone ? 0x2dd4bf : isCurrent ? 0x22d3ee : 0x6366f1);
        pod.bellMat.emissiveIntensity = isCurrent ? 0.68 : 0.28;
      }
    } else if (speciesId === 'tree_fold') {
      // BIFURCATING BINARY CORAL TREE: West `leaf(7)` vs East `branch(leaf(4), leaf(9))`
      const progress = stepState.progress ?? 0;
      const westActive = progress < 0.42 || isDone;
      const eastActive = progress >= 0.35 || isDone;
      const leafVals = entryMorph.treeLeaves || [7, 4, 9];
      for (const b of coralBranches) {
        const lit =
          b.def.side === 'root' ||
          (b.def.side === 'west' && westActive) ||
          (b.def.side.startsWith('east') && eastActive);
        b.mat.emissiveIntensity = lit ? 0.58 : 0.18;
      }
      for (let c = 0; c < coralPolypCrowns.length; c += 1) {
        const crown = coralPolypCrowns[c];
        const lit =
          (crown.side === 'west' && westActive) ||
          (crown.side.startsWith('east') && eastActive) ||
          isDone;
        const val = leafVals[c % leafVals.length] ?? 6;
        const baseScale = clamp(0.82 + val * 0.035, 0.8, 1.28);
        const pulse = baseScale * (lit ? 1.08 + 0.06 * Math.sin(ambient * 3.2 + c) : 0.85);
        crown.group.scale.setScalar(pulse);
        crown.mat.emissiveIntensity = lit ? 0.68 : 0.2;
      }
    } else if (speciesId === 'mutual_even_odd') {
      // TWIN-WINGED LUNA MOTH + RK4 LORENZ ORBIT: Left (`is_even`) & Right (`is_odd`) counter-flap
      const topCallName = activeCalls[activeCalls.length - 1]?.fnName || '';
      const evenActive = topCallName.includes('even') || stepIndex % 2 === 0;
      const flapAngle = Math.sin(ambient * 4.2 + q * Math.PI) * 0.46;
      const wingSpanScale = clamp(0.88 + (entryMorph.entryN ?? 4) * 0.03, 0.88, 1.22);
      leftWingPivot.scale.setScalar(wingSpanScale);
      rightWingPivot.scale.setScalar(wingSpanScale);
      leftWingPivot.rotation.y = flapAngle;
      rightWingPivot.rotation.y = -flapAngle;
      leftWingMat.emissiveIntensity = isDone ? 0.72 : evenActive ? 0.72 : 0.24;
      rightWingMat.emissiveIntensity = isDone ? 0.72 : !evenActive ? 0.72 : 0.24;
      lorenzOrbitMesh.rotation.z = Math.sin(ambient * 0.7) * 0.18;
    } else if (speciesId === 'lexical_let') {
      // 3 NESTED CRYSTAL CUBES IN UPWARD-CUPPED 24-PETAL LOTUS
      const activeScopeDepth = clamp(frameCount + (isDone ? 3 : 0), 1, 3);
      const seedScale = clamp(0.75 + 0.22 * Math.log10(1 + (entryMorph.seedValue ?? 12)), 0.72, 1.18);
      for (let c = 0; c < nestedCubes.length; c += 1) {
        const nc = nestedCubes[c];
        const dir = c % 2 === 0 ? 1 : -1;
        nc.group.scale.setScalar(seedScale);
        nc.group.rotation.y = ambient * 0.45 * dir + c * 0.5;
        nc.group.rotation.x = Math.PI * 0.25 + Math.sin(ambient * 0.8 + c) * 0.15;
        const lit = c < activeScopeDepth || isDone;
        nc.mat.emissiveIntensity = lit ? (isAluPrim ? 0.82 : 0.52) : 0.16;
      }
      for (let p = 0; p < lotusPetals.length; p += 1) {
        const lp = lotusPetals[p];
        const openMix = lp.ring < activeScopeDepth || isDone ? 1.0 : 0.4;
        lp.mesh.rotation.z = lerp(-0.38, -0.62 - lp.ring * 0.16, openMix) + Math.sin(ambient * 2 + p * 0.3) * 0.04;
        lp.mat.emissiveIntensity = openMix > 0.5 ? 0.48 : 0.18;
      }
    } else if (speciesId === 'checked_underflow') {
      // MEDUSA JELLYFISH BELL + 16 SINUOUS SPLINE TENTACLES
      const gapBoost = clamp((entryMorph.subGap ?? 6) / 16, 0.2, 1.2);
      const bellPulse = isFault ? 1.18 : 1.0 + (0.06 + 0.04 * gapBoost) * Math.sin(ambient * 3.2);
      jellyBellMesh.scale.set(bellPulse, 1.0 / Math.sqrt(bellPulse), bellPulse);
      jellyBellMat.color.setHex(isFault ? 0xe11d48 : 0xf43f5e);
      jellyBellMat.emissiveIntensity = isFault ? 0.82 : 0.42;

      for (let t = 0; t < JELLY_TENTACLE_COUNT; t += 1) {
        const baseA = (t / 16) * TAU;
        const baseRad = t % 2 === 0 ? 0.24 : 0.16;
        for (let s = 0; s < JELLY_TENTACLE_SEGMENTS; s += 1) {
          const u = s / (JELLY_TENTACLE_SEGMENTS - 1);
          const sway = Math.sin(ambient * 2.2 + t * 0.7 + u * 4.0) * (isFault ? 0.045 : 0.022) * u;
          const flare = isFault ? u * 0.14 : 0;
          const rad = baseRad + sway + flare;
          const twist = baseA + Math.cos(ambient * 1.6 + t + u * 3.0) * 0.12 * u;
          _tentaclePts[s].set(
            Math.cos(twist) * rad,
            lerp(0.02, -0.38, u),
            Math.sin(twist) * rad
          );
        }
        _color.setHex(isFault ? 0xe11d48 : t % 2 === 0 ? 0xfb7185 : 0xfbbf24);
        writeRibbonStrip(
          jellyTentaclePos,
          jellyTentacleCol,
          jellyTentacleNorm,
          t,
          JELLY_TENTACLE_SEGMENTS,
          _tentaclePts,
          0.0075,
          _color.r,
          _color.g,
          _color.b
        );
      }
      jellyTentacleGeom.attributes.position.needsUpdate = true;
      jellyTentacleGeom.attributes.color.needsUpdate = true;
      jellyTentacleGeom.attributes.normal.needsUpdate = true;
    } else if (speciesId === 'stack_overflow') {
      // BALLISTIC COMET + DYNAMIC `stackLimit` PANCAKED CEILING SHOCK RINGS
      const ringCount = clamp(artifact.analysis?.stackLimit ?? 5, 2, MAX_COMET_RINGS);
      const crushRatio = clamp(frameCount / ringCount, 0.15, 1.0);
      cometLanceSub.position.y = lerp(-0.08, 0.08, crushRatio);
      barrierMat.opacity = isFault ? 0.84 : 0.36 + crushRatio * 0.32;
      barrierMat.emissiveIntensity = isFault ? 0.95 : 0.48 + crushRatio * 0.3;

      for (let rIdx = 0; rIdx < MAX_COMET_RINGS; rIdx += 1) {
        const sr = cometShockRings[rIdx];
        if (rIdx >= ringCount) {
          sr.mesh.visible = false;
          continue;
        }
        sr.mesh.visible = true;
        const u = rIdx / Math.max(1, ringCount - 1);
        const yBase = lerp(-0.22, 0.28, u);
        const activeRing = rIdx < frameCount || isFault;
        const targetY = lerp(yBase, 0.32 - (ringCount - 1 - rIdx) * 0.032, crushRatio * 0.65);
        const scaleR = lerp(0.65, 1.45, u) * (activeRing ? 1.0 : 0.82);
        sr.mesh.position.y = targetY;
        sr.mesh.scale.set(scaleR, scaleR, 1.0 - u * 0.35);
        sr.mat.color.setHex(isFault ? 0xe11d48 : activeRing ? 0xf97316 : 0x64748b);
        sr.mat.emissiveIntensity = isFault ? 0.85 : activeRing ? 0.62 : 0.18;
      }
    } else if (speciesId === 'fuel_exhaustion') {
      // HOLLOW 14-SEGMENT TOKAMAK TORUS: 14 radial fuel cells extinguish one by one as fuel drains to 0
      const litCells = Math.round(liveFuelRatio * 14);
      torusBraidMesh.rotation.z = ambient * (0.15 + liveFuelRatio * 0.85);
      torusBraidMat.color.setHex(isExhausted ? 0x475569 : 0xfbbf24);
      torusBraidMat.emissiveIntensity = isExhausted ? 0.12 : 0.24 + liveFuelRatio * 0.48;
      for (let fIdx = 0; fIdx < torusFuelCells.length; fIdx += 1) {
        const cell = torusFuelCells[fIdx];
        const hasFuel = fIdx < litCells && !isExhausted;
        cell.mat.color.setHex(hasFuel ? 0x38bdf8 : 0x334155);
        cell.mat.emissive.setHex(hasFuel ? 0x0284c7 : 0x0f172a);
        cell.mat.emissiveIntensity = hasFuel ? 0.58 : 0.08;
        cell.mat.opacity = hasFuel ? 0.92 : 0.42;
      }
    }

    // ========================================================================
    // 4. UPDATE 64 RGB GENOME SPORES ON THE SPECIES SURFACE (`encodeColors`)
    // ========================================================================
    const rgbPixels = artifact.rgbPixels || [];
    for (let s = 0; s < SPORE_COUNT; s += 1) {
      const u = ((s / SPORE_COUNT) + ambient * 0.04 * (0.4 + liveFuelRatio)) % 1;
      const v = Math.sin(s * PHI_GOLDEN_ANGLE + ambient * 0.8);
      const surf = embedOnSpeciesSurface(speciesId, u, v, s % 16, 0.03);
      const outScale = isExhausted ? 0.88 : 1.08;

      _dummy.position.set(surf.x * outScale, surf.y * outScale, surf.z * outScale);
      _dummy.rotation.set(ambient + s, ambient * 0.7 + s, 0);
      _dummy.scale.setScalar(isDone ? 1.15 : isExhausted ? 0.5 : 0.85);
      _dummy.updateMatrix();
      sporeMesh.setMatrixAt(s, _dummy.matrix);

      const px = rgbPixels[s % Math.max(1, rgbPixels.length)];
      if (px && px.length >= 3) {
        _color.setRGB(px[0] / 255, px[1] / 255, px[2] / 255);
      } else {
        _color.setHex(species.primaryHex);
      }
      if (isFault) _color.lerp(new THREE.Color(0xe11d48), 0.7);
      sporeMesh.setColorAt(s, _color);
    }
    sporeMesh.instanceMatrix.needsUpdate = true;
    if (sporeMesh.instanceColor) sporeMesh.instanceColor.needsUpdate = true;

    // ========================================================================
    // 5. UPDATE VERTICAL CHAMBER TENDRILS (Starting Outside the Organism Envelope)
    // ========================================================================
    const floorRelY = -3.38 - QUINELING_CORE_Y; // -1.33
    const ringsRelY = -0.25 - QUINELING_CORE_Y; // +1.80
    for (let c = 0; c < TENDRIL_CURVE_COUNT; c += 1) {
      const isFloorRoot = c < 8;
      const idx = c % 8;
      const angle = (idx / 8) * TAU + ambient * 0.1;
      const startR = QUINELING_MAX_RADIUS + 0.04; // Starts outside the body so rays never stab through the center!
      const endR = isFloorRoot ? 0.56 : 0.46;
      const startY = isFloorRoot ? -0.26 : 0.26;
      const endY = isFloorRoot ? floorRelY : ringsRelY;

      for (let s = 0; s < TENDRIL_SEGMENTS; s += 1) {
        const t = s / (TENDRIL_SEGMENTS - 1);
        const rad = lerp(startR, endR, t) + Math.sin(t * Math.PI) * 0.035;
        const twist = angle + t * (isFloorRoot ? -0.35 : 0.35);
        _tendrilPts[s].set(
          Math.cos(twist) * rad,
          lerp(startY, endY, t),
          Math.sin(twist) * rad
        );
      }
      _color.setHex(isFault ? 0xe11d48 : isDone ? 0x2dd4bf : isFloorRoot ? species.primaryHex : species.secondaryHex);
      writeRibbonStrip(
        tendrilPositions,
        tendrilColors,
        tendrilNormals,
        c,
        TENDRIL_SEGMENTS,
        _tendrilPts,
        0.0036,
        _color.r,
        _color.g,
        _color.b
      );
    }
    tendrilGeom.attributes.position.needsUpdate = true;
    tendrilGeom.attributes.color.needsUpdate = true;
    tendrilGeom.attributes.normal.needsUpdate = true;
  }

  function controlKindIsReturn(step) {
    const ck = step?.after?.control?.kind || step?.controlKind || '';
    return ck === 'Return' || ck === 'Done';
  }

  return {
    group,
    coreRoot,
    speciesGroups,
    update,
  };
}
