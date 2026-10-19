import test from 'node:test';
import assert from 'node:assert/strict';
import fc from 'fast-check';

import {
  PRESET_PROGRAMS,
  parseAndCheckDraft,
  compileAndRunInBrowser,
  compileAndRunSherlock,
  buildStepState,
  buildRingTelemetry,
  buildRecursionTelemetry,
  materializeArithmeticWitness,
  materializeFuelWitness,
  clearClientTraceCache,
} from '../src/engine/sherlockEngine.js';

import {
  compileSherlockProgramToQuineling,
  clearCompiledQuinelingCache,
} from '../src/engine/sherlockQuinelingCompiler.js';

import {
  createQuinelingSubsystem,
  QUINELING_SPECIES,
  getQuinelingTelemetry,
} from '../src/designers/designer8_quineling.js';

import { createPackageSubsystem } from '../src/designers/designer1_package.js';
import { createWellSubsystem } from '../src/designers/designer2_well.js';
import { createFramesSubsystem } from '../src/designers/designer3_frames.js';
import { createArenaSubsystem } from '../src/designers/designer4_arena.js';
import { createCircuitSubsystem } from '../src/designers/designer5_circuit.js';
import { createLoomSubsystem } from '../src/designers/designer6_loom.js';
import { disposeThemeCaches } from '../src/designers/theme.js';

const BURN_IN_MULTIPLIER = Number(process.env.BURN_IN_MULTIPLIER || 1);

// ---------------------------------------------------------------------------
// Minimal Offscreen DOM / Canvas 2D Polyfill for Node.js Headless Fuzzing of
// Three.js Designers & `theme.js` CanvasTextures
// ---------------------------------------------------------------------------
function installHeadlessCanvasPolyfill() {
  if (typeof globalThis.document !== 'undefined') return;
  const noop = () => {};
  const createMockCtx2D = () => ({
    fillStyle: '',
    strokeStyle: '',
    lineWidth: 1,
    font: '',
    textAlign: 'left',
    textBaseline: 'alphabetic',
    shadowColor: '',
    shadowBlur: 0,
    globalAlpha: 1,
    clearRect: noop,
    fillRect: noop,
    strokeRect: noop,
    beginPath: noop,
    closePath: noop,
    moveTo: noop,
    lineTo: noop,
    arc: noop,
    arcTo: noop,
    rect: noop,
    roundRect: noop,
    drawImage: noop,
    ellipse: noop,
    quadraticCurveTo: noop,
    bezierCurveTo: noop,
    fill: noop,
    stroke: noop,
    clip: noop,
    save: noop,
    restore: noop,
    translate: noop,
    rotate: noop,
    scale: noop,
    setLineDash: noop,
    fillText: noop,
    strokeText: noop,
    measureText: (str) => ({ width: String(str || '').length * 8 }),
    createLinearGradient: () => ({ addColorStop: noop }),
    createRadialGradient: () => ({ addColorStop: noop }),
    getImageData: (_x, _y, w, h) => ({ data: new Uint8ClampedArray(w * h * 4) }),
    putImageData: noop,
  });

  globalThis.document = {
    createElement(tag) {
      if (tag === 'canvas') {
        return {
          width: 256,
          height: 256,
          style: {},
          getContext: () => createMockCtx2D(),
        };
      }
      return { style: {}, setAttribute: noop, appendChild: noop };
    },
  };
}

installHeadlessCanvasPolyfill();

// ---------------------------------------------------------------------------
// Grammar-Guided Arbitraries for `moriarty-sherlock-draft/1` Programs
// ---------------------------------------------------------------------------
const arbUIntLiteral = fc.oneof(
  fc.integer({ min: 0, max: 25 }),
  fc.integer({ min: 0, max: 10000 }),
  fc.constantFrom(0, 1, 2, 3, 4, 5, 7, 8, 9, 12, 15, 255, 256, 65535)
);

const arbValidProgram = fc
  .record({
    templateIdx: fc.integer({ min: 0, max: PRESET_PROGRAMS.length - 1 }),
    litA: fc.integer({ min: 0, max: 9 }),
    litB: fc.integer({ min: 1, max: 12 }),
    litC: fc.integer({ min: 0, max: 15 }),
    fuel: fc.integer({ min: 4, max: 240 }),
    stackLimit: fc.integer({ min: 2, max: 24 }),
    sliceWidth: fc.integer({ min: 1, max: 8 }),
  })
  .map(({ templateIdx, litA, litB, litC, fuel, stackLimit, sliceWidth }) => {
    const base = PRESET_PROGRAMS[templateIdx];
    let source = base.source;
    switch (base.id) {
      case 'factorial':
        source = `language "moriarty-sherlock-draft/1";
program FactorialFuzz {
  fn fact(n: UInt): UInt {
    if n == 0 {
      1
    } else {
      n * fact(n - 1)
    }
  }
  entry fact(${litA % 7});
}`;
        break;
      case 'list_fold':
        source = `language "moriarty-sherlock-draft/1";
program ListFoldFuzz {
  fn sum_list(xs: List): UInt {
    match xs {
      nil => 0,
      cons(h, t) => h + sum_list(t)
    }
  }
  entry sum_list(cons(${litA}, cons(${litB}, cons(${litC}, nil))));
}`;
        break;
      case 'tree_fold':
        source = `language "moriarty-sherlock-draft/1";
program TreeFoldFuzz {
  fn sum_tree(t: Tree): UInt {
    match t {
      leaf(x) => x,
      branch(l, r) => sum_tree(l) + sum_tree(r)
    }
  }
  entry sum_tree(branch(leaf(${litA}), branch(leaf(${litB}), leaf(${litC}))));
}`;
        break;
      case 'mutual_even_odd':
        source = `language "moriarty-sherlock-draft/1";
program MutualParityFuzz {
  fn is_even(n: UInt): Bool {
    if n == 0 {
      true
    } else {
      is_odd(n - 1)
    }
  }
  fn is_odd(n: UInt): Bool {
    if n == 0 {
      false
    } else {
      is_even(n - 1)
    }
  }
  entry is_even(${litA % 8});
}`;
        break;
      case 'lexical_let':
        source = `language "moriarty-sherlock-draft/1";
program LexicalLetFuzz {
  fn compute(x: UInt, bonus: UInt): UInt {
    let sq = x * x in
    let cube = sq * x in
    cube + bonus
  }
  entry compute(${litB}, ${litC});
}`;
        break;
      case 'checked_underflow':
        source = `language "moriarty-sherlock-draft/1";
program CheckedSubFuzz {
  fn sub_pair(a: UInt, b: UInt): UInt {
    a - b
  }
  entry sub_pair(${litA}, ${litB});
}`;
        break;
      case 'stack_overflow':
        source = `language "moriarty-sherlock-draft/1";
program DeepFrameFuzz {
  fn dive(n: UInt): UInt {
    if n == 0 {
      0
    } else {
      1 + dive(n - 1)
    }
  }
  entry dive(${litB});
}`;
        break;
      case 'fuel_exhaustion':
        source = `language "moriarty-sherlock-draft/1";
program FuelBoundFuzz {
  fn spin(n: UInt): UInt {
    if n == 0 {
      0
    } else {
      spin(n - 1)
    }
  }
  entry spin(${litB});
}`;
        break;
      default:
        break;
    }
    return {
      presetId: base.id,
      source,
      fuel,
      stackLimit,
      sliceWidth,
    };
  });

const arbHostileSource = fc.oneof(
  fc.string({ maxLength: 300 }),
  fc
    .tuple(
      fc.integer({ min: 0, max: PRESET_PROGRAMS.length - 1 }),
      fc.integer({ min: 0, max: 200 }),
      fc.string({ minLength: 1, maxLength: 20 })
    )
    .map(([idx, pos, noise]) => {
      const s = PRESET_PROGRAMS[idx].source;
      const cut = pos % Math.max(1, s.length);
      return s.slice(0, cut) + noise + s.slice(cut);
    })
);

// ---------------------------------------------------------------------------
// 1. Parser, Typechecker & 39-Rule CEK Machine Fuzzing
// ---------------------------------------------------------------------------
test('Fuzz 1: Hostile & Mutated Source Inputs Never Crash Parser or CEK Engine', () => {
  fc.assert(
    fc.property(
      arbHostileSource,
      fc.integer({ min: -5, max: 300 }),
      fc.integer({ min: -2, max: 32 }),
      fc.integer({ min: -1, max: 16 }),
      (rawSource, fuel, stackLimit, sliceWidth) => {
        const res = compileAndRunInBrowser(rawSource, { fuel, stackLimit, sliceWidth });
        assert.equal(typeof res, 'object');
        assert.equal(typeof res.ok, 'boolean');
        if (!res.ok) {
          assert.equal(typeof res.error, 'string');
          assert.ok(res.error.length > 0);
        } else {
          assert.ok(Array.isArray(res.steps));
          assert.ok(Array.isArray(res.slices));
        }
      }
    ),
    { numRuns: 300 * BURN_IN_MULTIPLIER }
  );
});

test('Fuzz 2: Valid Grammar-Generated Sherlock Programs Satisfy All 39-Rule CEK Invariants', () => {
  fc.assert(
    fc.property(
      arbValidProgram,
      fc.float({ min: -5, max: 300, noNaN: true, noDefaultInfinity: true }),
      (prog, rawPlayhead) => {
        const trace = compileAndRunInBrowser(prog.source, {
          fuel: prog.fuel,
          stackLimit: prog.stackLimit,
          sliceWidth: prog.sliceWidth,
        });
        assert.equal(trace.ok, true, `Expected valid program to compile: ${trace.error}`);
        assert.ok(trace.steps.length >= 1, 'Expected at least 1 CEK transition step');

        // Check every step's fuel conservation, stack limit bound, and witness arrays
        for (let i = 0; i < trace.steps.length; i++) {
          const step = trace.steps[i];
          assert.equal(step.index, i);
          assert.ok(step.after.fuel <= step.before.fuel, 'Fuel must be monotonically non-increasing');
          assert.equal(
            step.before.fuel - step.after.fuel,
            step.charge,
            'Fuel delta must equal step charge'
          );
          assert.ok(
            step.after.frames.length <= prog.stackLimit,
            `Frame count ${step.after.frames.length} exceeded stackLimit ${prog.stackLimit}`
          );
          assert.equal(step.after.aux2.arithmetic.lhsBytes.length, 16);
          assert.equal(step.after.aux2.arithmetic.rhsBytes.length, 16);
          assert.equal(step.after.aux2.arithmetic.resultBytes.length, 16);
          assert.equal(step.after.aux2.arithmetic.carries.length, 17);
          assert.equal(step.after.aux2.arithmetic.borrows.length, 17);
          assert.equal(step.after.aux2.arithmetic.diagonals.length, 32);
        }

        // Verify buildStepState, buildRingTelemetry, and buildRecursionTelemetry at arbitrary playhead
        const stepState = buildStepState({
          trace,
          playhead: rawPlayhead,
          ambient: 1.23,
          dt: 0.016,
        });
        assert.ok(stepState.stepIndex >= 0 && stepState.stepIndex < trace.steps.length);
        assert.ok(Number.isFinite(stepState.stepFraction));
        assert.ok(Number.isFinite(stepState.progress) && stepState.progress >= 0 && stepState.progress <= 1);
        assert.equal(stepState.ringTelemetry.rings.length, 6);
        assert.ok(
          stepState.controlCore.sectorIndex >= 0 && stepState.controlCore.sectorIndex < 10
        );
      }
    ),
    { numRuns: 250 * BURN_IN_MULTIPLIER }
  );
});

test('Fuzz 3: Edge-Case 0-Step & 1-Step Traces + 128-Bit Arithmetic Witnesses', () => {
  // 0-step and 1-step synthetic trace invariants
  for (const syntheticSteps of [[], [compileAndRunInBrowser(PRESET_PROGRAMS[0].source).steps[0]]]) {
    const trace = { ok: true, steps: syntheticSteps, outcome: { kind: 'Done' } };
    for (const playhead of [-10, 0, 0.5, 1, 100]) {
      const st = buildStepState({ trace, playhead, ambient: 0, dt: 0.016 });
      assert.ok(st.stepIndex >= 0);
      assert.ok(Number.isFinite(st.progress));
      assert.ok(Number.isFinite(st.stepFraction));
    }
  }

  // 128-bit arithmetic witness fuzzing across +, -, *, <, ==
  fc.assert(
    fc.property(
      fc.constantFrom('+', '-', '*', '<', '==', null),
      fc.oneof(fc.bigInt({ min: 0n, max: (1n << 128n) - 1n }), fc.constant(null), fc.constant(undefined)),
      fc.oneof(fc.bigInt({ min: 0n, max: (1n << 128n) - 1n }), fc.constant(null), fc.constant(undefined)),
      (op, lhs, rhs) => {
        const w = materializeArithmeticWitness(op, lhs, rhs);
        assert.equal(typeof w.active, 'boolean');
        assert.equal(w.lhsBytes.length, 16);
        assert.equal(w.rhsBytes.length, 16);
        assert.equal(w.resultBytes.length, 16);
        for (let i = 0; i < 16; i++) {
          assert.ok(w.resultBytes[i] >= 0 && w.resultBytes[i] <= 255);
        }
      }
    ),
    { numRuns: 300 * BURN_IN_MULTIPLIER }
  );
});

// ---------------------------------------------------------------------------
// 2. Differential Fuzzing: Browser JS Engine vs. Native Rust `sherlock-runner`
// ---------------------------------------------------------------------------
test('Fuzz 4: Differential Fuzzing — Browser JS CEK Engine Matches Native Rust sherlock-runner 100%', async () => {
  await fc.assert(
    fc.asyncProperty(arbValidProgram, async (prog) => {
      const jsTrace = compileAndRunInBrowser(prog.source, {
        fuel: prog.fuel,
        stackLimit: prog.stackLimit,
        sliceWidth: prog.sliceWidth,
      });

      const resp = await fetch('http://127.0.0.1:4377/api/trace', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          source: prog.source,
          fuel: prog.fuel,
          stackLimit: prog.stackLimit,
          sliceWidth: prog.sliceWidth,
        }),
      });
      assert.equal(resp.status, 200);
      const rustTrace = await resp.json();

      assert.equal(rustTrace.ok, jsTrace.ok, 'JS and Rust ok status must match');
      assert.equal(
        rustTrace.outcome.kind,
        jsTrace.outcome.kind,
        `Outcome mismatch on ${prog.presetId}: Rust=${rustTrace.outcome.kind} vs JS=${jsTrace.outcome.kind}`
      );
      assert.equal(
        rustTrace.steps.length,
        jsTrace.steps.length,
        `Step count mismatch on ${prog.presetId}`
      );

      for (let i = 0; i < jsTrace.steps.length; i++) {
        const jsStep = jsTrace.steps[i];
        const rsStep = rustTrace.steps[i];
        assert.equal(rsStep.ruleLine, jsStep.ruleLine, `Rule line mismatch at step ${i}`);
        assert.equal(rsStep.ruleName, jsStep.ruleName, `Rule name mismatch at step ${i}`);
        assert.equal(rsStep.charge, jsStep.charge, `Charge mismatch at step ${i}`);
        assert.equal(rsStep.after.fuel, jsStep.after.fuel, `Fuel mismatch at step ${i}`);
        assert.equal(
          rsStep.after.frames.length,
          jsStep.after.frames.length,
          `Frame stack depth mismatch at step ${i}`
        );
      }
    }),
    { numRuns: 100 * BURN_IN_MULTIPLIER }
  );
});

// ---------------------------------------------------------------------------
// 3. Property-Based Fuzzing: Quineling Compiler, Upstream SDK & WovenBody
// ---------------------------------------------------------------------------
test('Fuzz 5: Quineling Compiler & Upstream WovenBody Satisfy Quine Identity, Bijective Mechanisms & Finite Geometry', () => {
  // First verify the 8 canonical presets map bijectively to the 8 nursery.html mechanisms
  const seenMechanisms = new Set();
  for (const preset of PRESET_PROGRAMS) {
    const trace = compileAndRunInBrowser(preset.source, {
      fuel: preset.defaultFuel,
      stackLimit: preset.defaultStackLimit,
      sliceWidth: 4,
    });
    const artifact = compileSherlockProgramToQuineling(preset.source, QUINELING_SPECIES, trace);
    assert.equal(artifact.quineVerified, true, `Quine verification failed for ${preset.id}`);
    assert.equal(
      artifact.wovenRecord.mechanism,
      QUINELING_SPECIES[preset.id].mechanism,
      `Mechanism mismatch for ${preset.id}`
    );
    seenMechanisms.add(artifact.wovenRecord.mechanism);
  }
  assert.equal(seenMechanisms.size, 8, 'All 8 nursery.html mechanisms must be bijectively represented');

  // Now fuzz arbitrary programs, phases, budgets, and undersized reuse buffers
  const sharedReuse = {
    points: new Float32Array(16), // intentionally undersized initially
    owners: new Uint16Array(4),
  };

  fc.assert(
    fc.property(
      arbValidProgram,
      fc.double({ min: -1000, max: 1000, noNaN: true, noDefaultInfinity: true }),
      fc.integer({ min: 128, max: 2400 }),
      fc.boolean(),
      (prog, phase, budget, crests) => {
        const trace = compileAndRunInBrowser(prog.source, {
          fuel: prog.fuel,
          stackLimit: prog.stackLimit,
          sliceWidth: prog.sliceWidth,
        });
        const artifact = compileSherlockProgramToQuineling(prog.source, QUINELING_SPECIES, trace);

        assert.equal(artifact.quineVerified, true);
        assert.ok(artifact.quineBytes > 0);
        assert.ok(artifact.harmonicGenome && artifact.colorGenome);
        assert.ok(artifact.wovenBounds.width > 0 && Number.isFinite(artifact.wovenBounds.width));
        assert.ok(artifact.wovenBounds.height > 0 && Number.isFinite(artifact.wovenBounds.height));

        const nodeCount = artifact.shape.nodes.length;
        const frame = globalThis.WovenBody.frame(artifact.wovenBody, phase, {
          budget,
          crests,
          reuse: sharedReuse,
        });

        assert.equal(frame.points.length, budget * 4);
        assert.equal(frame.owners.length, budget);
        assert.equal(sharedReuse.points, frame.points, 'reuse.points must be updated in place');
        assert.equal(sharedReuse.owners, frame.owners, 'reuse.owners must be updated in place');

        for (let i = 0; i < frame.points.length; i++) {
          assert.ok(
            Number.isFinite(frame.points[i]),
            `Non-finite coordinate/alpha at index ${i} for ${artifact.wovenRecord.mechanism}`
          );
        }
        for (let i = 0; i < frame.owners.length; i++) {
          assert.ok(
            frame.owners[i] >= 0 && frame.owners[i] < nodeCount,
            `Out-of-bounds owner ${frame.owners[i]} (nodeCount=${nodeCount})`
          );
        }
      }
    ),
    { numRuns: 120 * BURN_IN_MULTIPLIER }
  );
});

// ---------------------------------------------------------------------------
// 4. Stateful Model-Based Burn-In Fuzzing of Three.js Designers & Lifecycle
// ---------------------------------------------------------------------------
test('Fuzz 6: Stateful Burn-In of Three.js Designers Across Random Scrubbing, Presets & Disposal', () => {
  const designers = [
    createPackageSubsystem(),
    createWellSubsystem(),
    createFramesSubsystem(),
    createArenaSubsystem(),
    createCircuitSubsystem(),
    createLoomSubsystem(),
    createQuinelingSubsystem(),
  ];

  const tracesByPreset = new Map();
  for (const p of PRESET_PROGRAMS) {
    tracesByPreset.set(
      p.id,
      compileAndRunInBrowser(p.source, {
        fuel: p.defaultFuel,
        stackLimit: p.defaultStackLimit,
        sliceWidth: 4,
      })
    );
  }

  let ambient = 0;
  fc.assert(
    fc.property(
      fc.integer({ min: 0, max: PRESET_PROGRAMS.length - 1 }),
      fc.double({ min: 0, max: 80, noNaN: true, noDefaultInfinity: true }),
      fc.double({ min: 0.001, max: 0.05, noNaN: true, noDefaultInfinity: true }),
      (presetIdx, rawPlayhead, dt) => {
        const preset = PRESET_PROGRAMS[presetIdx];
        const trace = tracesByPreset.get(preset.id);
        ambient += dt;

        const stepState = buildStepState({
          trace,
          playhead: rawPlayhead,
          ambient,
          dt,
        });
        stepState.presetId = preset.id;
        stepState.source = preset.source;

        for (const d of designers) {
          d.update(stepState, ambient, dt);
        }

        const tel = getQuinelingTelemetry({
          presetId: preset.id,
          source: preset.source,
          trace,
          step: stepState.step,
          stepIndex: stepState.stepIndex,
        });
        assert.ok(tel.speciesName.length > 0);
        assert.ok(tel.mechanism.length > 0);
      }
    ),
    { numRuns: 200 * BURN_IN_MULTIPLIER }
  );

  // Verify clean disposal of all designers and theme caches without throwing
  for (const d of designers) {
    if (typeof d.dispose === 'function') {
      d.dispose();
    }
  }
  disposeThemeCaches();
  clearCompiledQuinelingCache();
  clearClientTraceCache();
});
