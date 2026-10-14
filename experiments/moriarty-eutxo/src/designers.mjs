import { readFileSync, readdirSync, existsSync, writeFileSync, mkdirSync } from 'node:fs';
import { join, basename, dirname } from 'node:path';
import { createHash } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import { compile } from '../../moriarty-language/src/frontend.ts';
import { createSimulator } from '../../moriarty-language/src/evaluate.ts';
import { createFinancialAgreementSourceV5 } from '../../moriarty-language/src/successor/financial-agreement-source-v5.ts';
import { createFinancialAgreementSourceV6 } from '../../moriarty-language/src/successor/financial-agreement-source-v6.ts';
import { canonical } from '../../moriarty-language/src/successor/expression-wire-v1.ts';
import { admitPublicAtomic, detectProfile, PUBLIC_PROFILE } from './admit.mjs';
import { BOUNDS, compilePublic } from './compile-public.mjs';

const PROTOTYPE_SOURCE = 'revision-step.mori';
const KNOWN_LIMITS = new Map([
  ['E06-refund-boundary', { status: 'Rejected', code: 'SELF_TRANSFER' }],
  ['E08-cancel-residual', { status: 'Rejected', code: 'SELF_TRANSFER' }],
  ['E09-refund-unlock-only', { status: 'Rejected', code: 'EMPTY_BATCH' }],
  ['E10-cancel-preserves-residual', { status: 'Rejected', code: 'EMPTY_BATCH' }],
]);

function readJson(path) {
  return JSON.parse(readFileSync(path, 'utf8'));
}

function sha256File(path) {
  return createHash('sha256').update(readFileSync(path)).digest('hex');
}

function listMori(dir) {
  return readdirSync(dir).filter(f => f.endsWith('.mori')).map(f => join(dir, f));
}

function loadExpected(dir, expected) {
  if (expected == null) return null;
  if (typeof expected === 'string' && expected.endsWith('.json')) {
    const p = join(dir, expected);
    return existsSync(p) ? readJson(p) : { missingFile: expected };
  }
  return expected;
}

function evaluateSuccessor(profile, source, action, ordinary, financial) {
  const api = profile.endsWith('/5') ? createFinancialAgreementSourceV5() : createFinancialAgreementSourceV6();
  const snap = typeof ordinary === 'string' ? ordinary : canonical(ordinary);
  const fin = typeof financial === 'string' ? financial : JSON.stringify(financial);
  return api.evaluate(source, action, snap, fin);
}

function evaluateAtomic(source, input) {
  const sim = createSimulator(source, BOUNDS);
  return sim.simulate(input);
}

function sourceStatus(result) {
  if (!result) return { tier: 'source', status: 'not-run' };
  if (result.outcome === 'Rejected' || result.status === 'Rejected') {
    return { tier: 'source', status: 'Rejected', code: result.code || result.diagnostics?.[0]?.code || null, actual: summarizeActual(result) };
  }
  if (result.kind === 'Simulation' || result.outcome === 'Complete' || result.status === 'FundedExpressionPrepared') {
    return { tier: 'source', status: result.status || result.outcome || 'Complete', actual: summarizeActual(result) };
  }
  if (result.judgmentResult) return { tier: 'source', status: result.judgmentResult, actual: result };
  return { tier: 'source', status: result.status || 'unknown', actual: summarizeActual(result) };
}

function summarizeActual(result) {
  if (result?.kind === 'Simulation') {
    return {
      outcome: result.candidate.body.outcome,
      revision: result.candidate.body.after.body.revision,
      remaining: result.candidate.body.after.body.remaining,
      effects: result.candidate.body.effects.map(e => ({ kind: e.kind, from: e.from, to: e.to, amount: e.amount?.value, asset: e.asset })),
    };
  }
  if (result?.status) {
    return {
      status: result.status,
      code: result.code || null,
      post: result.post || null,
      effects: result.effects || null,
      workRemaining: result.workRemaining || null,
    };
  }
  return { keys: result && Object.keys(result) };
}

function envelopesEqual(actual, expected) {
  if (!expected || expected.missingFile) return { compared: false };
  if (expected.status === 'Rejected' || expected.code) {
    const statusOk = !expected.status || actual.status === expected.status;
    const codeOk = !expected.code || actual.code === expected.code;
    return { compared: true, match: statusOk && codeOk };
  }
  if (expected.status === 'FundedExpressionPrepared' || expected.post || expected.financialPost) {
    try {
      const subset = {
        status: actual.actual?.status || actual.status,
        post: actual.actual?.post ?? null,
        effects: actual.actual?.effects ?? null,
        workRemaining: actual.actual?.workRemaining ?? null,
      };
      const want = {
        status: expected.status,
        post: expected.post ?? subset.post,
        effects: expected.effects ?? subset.effects,
        workRemaining: expected.workRemaining ?? subset.workRemaining,
      };
      return { compared: true, match: JSON.stringify(subset) === JSON.stringify(want) };
    } catch {
      return { compared: true, match: false };
    }
  }
  return { compared: false };
}

function collectPatterns(dir, cases) {
  const designer = basename(dir);
  const patterns = [];
  if (Array.isArray(cases.patterns) && cases.patterns[0]?.id) {
    for (const p of cases.patterns) patterns.push({ ...p, designer });
  } else if (Array.isArray(cases.patterns) && typeof cases.patterns[0] === 'string') {
    for (const id of cases.patterns) {
      const prefix = String(id).split('-')[0];
      const related = (cases.cases || []).filter(c => c.id === id || c.pattern === id || (prefix && String(c.id).startsWith(prefix + '-')));
      patterns.push({ id, name: id, designer, relatedCases: related });
    }
  }
  if (Array.isArray(cases.cases) && !patterns.length) {
    const byPattern = new Map();
    for (const c of cases.cases) {
      const key = c.pattern || c.id;
      if (!byPattern.has(key)) byPattern.set(key, { id: key, name: key, designer, relatedCases: [] });
      byPattern.get(key).relatedCases.push(c);
    }
    patterns.push(...byPattern.values());
  }
  return patterns;
}

function resolveSourceNames(pattern) {
  const names = [];
  if (typeof pattern.source === 'string') names.push(pattern.source);
  if (Array.isArray(pattern.sources)) names.push(...pattern.sources);
  if (pattern.source && typeof pattern.source === 'object' && typeof pattern.source.source === 'string') {
    names.push(pattern.source.source);
  }
  return [...new Set(names.filter(n => typeof n === 'string'))];
}

function nestedSourceDescriptor(pattern, pack) {
  if (pattern.source && typeof pattern.source === 'object' && typeof pattern.source.source === 'string') {
    return pattern.source;
  }
  if (typeof pattern.source === 'string') return { source: pattern.source };
  const sibling = pack?.patterns?.find(p => p.source);
  if (!sibling) return null;
  if (typeof sibling.source === 'object' && typeof sibling.source.source === 'string') return sibling.source;
  if (typeof sibling.source === 'string') return { source: sibling.source };
  return null;
}

function resolveSource(dir, pattern, pack) {
  const names = resolveSourceNames(pattern);
  if (!names.length) {
    const nested = nestedSourceDescriptor(pattern, pack);
    if (nested?.source) names.push(nested.source);
  }
  return [...new Set(names)].map(n => join(dir, n)).filter(existsSync);
}

function resolveOrdinary(dir, pattern) {
  if (typeof pattern.ordinaryInputCanonicalJSON === 'string' && pattern.ordinaryInputCanonicalJSON.trim().startsWith('{')) {
    return pattern.ordinaryInputCanonicalJSON.trim();
  }
  const candidates = [
    pattern.ordinary,
    pattern.input,
    pattern.inputs?.ordinary,
    pattern.source?.ordinary,
    pattern.source?.input,
  ].filter(x => typeof x === 'string');
  for (const name of candidates) {
    const p = join(dir, name);
    if (existsSync(p)) return readFileSync(p, 'utf8').trim();
  }
  return null;
}

function resolveFinancial(dir, pattern) {
  const candidates = [
    pattern.financial,
    pattern.financialInput,
    pattern.financialPre,
    pattern.inputs?.financial,
    pattern.source?.financial,
  ].filter(x => typeof x === 'string');
  for (const name of candidates) {
    const p = join(dir, name);
    if (existsSync(p)) return readFileSync(p, 'utf8');
  }
  return null;
}

function runSourceCase(dir, pattern, sourcePath) {
  const source = readFileSync(sourcePath, 'utf8');
  const profile = detectProfile(source) || pattern.profile || pattern.sourceProfile || pattern.source?.profile || null;
  const action = pattern.action || pattern.source?.action || null;
  try {
    if (profile === PUBLIC_PROFILE) {
      const ordinary = resolveOrdinary(dir, pattern);
      if (ordinary) {
        const input = ordinary.trim().startsWith('{') ? JSON.parse(ordinary) : null;
        if (input?.schemaVersion === 'moriarty-evaluation/1') return sourceStatus(evaluateAtomic(source, input));
      }
      compile(source, BOUNDS);
      return { tier: 'source', status: 'compiled', profile, note: 'atomic compile succeeded; full simulate skipped without evaluation input' };
    }
    if (profile?.startsWith('moriarty-financial-agreement-source/')) {
      const probeName = pattern.sourceLimitationTests?.probe;
      const isProbe = probeName && basename(sourcePath) === basename(probeName);
      if (isProbe) {
        const api = profile.endsWith('/5') ? createFinancialAgreementSourceV5() : createFinancialAgreementSourceV6();
        const checked = api.check(source);
        return {
          ...sourceStatus(checked),
          note: checked.code === 'OPERATION_BINDING'
            ? 'unsupported source check OPERATION_BINDING (distinct from SOURCE_ACTION_UNKNOWN)'
            : 'source probe check',
        };
      }
      const ordinary = resolveOrdinary(dir, pattern);
      const financial = resolveFinancial(dir, pattern);
      if (action && ordinary && financial) return sourceStatus(evaluateSuccessor(profile, source, action, ordinary, financial));
      if (Array.isArray(pattern.sourceLimitationTests?.actionNames) && !action) {
        const results = pattern.sourceLimitationTests.actionNames.map(name => {
          const api = profile.endsWith('/5') ? createFinancialAgreementSourceV5() : createFinancialAgreementSourceV6();
          const r = api.evaluate(source, name, ordinary || '{"Args":{},"Obs":{},"Pre":{},"workInitial":"512"}', financial || '{}');
          return { action: name, status: r.status, code: r.code || null };
        });
        return {
          tier: 'source',
          status: 'SOURCE_ACTION_UNKNOWN',
          code: 'SOURCE_ACTION_UNKNOWN',
          actions: results,
          note: 'unsupported selector distinct from OPERATION_BINDING probe',
        };
      }
      const api = profile.endsWith('/5') ? createFinancialAgreementSourceV5() : createFinancialAgreementSourceV6();
      const checked = api.check(source);
      if (checked.status === 'Rejected') {
        return {
          ...sourceStatus(checked),
          note: checked.code === 'OPERATION_BINDING'
            ? 'unsupported source check OPERATION_BINDING (distinct from SOURCE_ACTION_UNKNOWN)'
            : undefined,
        };
      }
      return { tier: 'source', status: 'skipped-incomplete-input', profile, note: 'unknown source skip is not success; no complete ordinary/financial pair for evaluate' };
    }
    return { tier: 'source', status: 'PROFILE_UNSUPPORTED', profile, note: 'unknown source skip is not success' };
  } catch (e) {
    return { tier: 'source', status: 'Rejected', code: e.code || e.name, message: e.message };
  }
}

function uplcStatus(sourcePath, manifest, compiledExample) {
  const source = readFileSync(sourcePath, 'utf8');
  const profile = detectProfile(source);
  if (profile !== PUBLIC_PROFILE) return { tier: 'UPLC', status: 'unsupported', code: 'PROFILE_UNSUPPORTED', profile };
  try {
    const bound = compile(source, BOUNDS).bound;
    const admitted = admitPublicAtomic(bound, manifest);
    if (!admitted.ok) return { tier: 'UPLC', status: 'unsupported', code: admitted.code, detail: { code: admitted.code } };
    const isPrototype = basename(sourcePath) === PROTOTYPE_SOURCE
      || (compiledExample?.programHash && bound.programHash === compiledExample.programHash);
    if (!isPrototype) {
      return {
        tier: 'UPLC',
        status: 'admitted-not-compiled',
        code: null,
        programHash: bound.programHash,
        note: 'actual compiled UPLC is recorded only for the tested prototype/admitted fixtures',
      };
    }
    if (compiledExample?.ok) {
      return {
        tier: 'UPLC',
        status: compiledExample.positiveUplcValidation ? 'compiled-public-atomic' : 'compiler-accepted-no-positive-fixture',
        programHash: bound.programHash,
        checkStatus: compiledExample.checkStatus,
      };
    }
    return { tier: 'UPLC', status: 'admitted-public-atomic', code: null, programHash: bound.programHash };
  } catch (e) {
    return { tier: 'UPLC', status: 'Rejected', code: e.code || e.name, message: e.message };
  }
}

function classifyMatch(pattern, src, expected) {
  const known = KNOWN_LIMITS.get(pattern.id);
  if (known && src.status === known.status && src.code === known.code) {
    return { expectedMatch: true, knownLimit: known.code, note: 'named designer limitation; frozen cases.json expected is obsolete for this id' };
  }
  if (pattern.expectedSourceLimitation && src.status === pattern.expectedSourceLimitation.status && src.code === pattern.expectedSourceLimitation.code) {
    return { expectedMatch: true, knownLimit: src.code, note: 'named expectedSourceLimitation' };
  }
  if (Array.isArray(pattern.sourceExpected) && src.expectedCode) {
    return { expectedMatch: src.code === src.expectedCode, knownLimit: null };
  }
  if (pattern.sourceLimitationTests?.expectedCode === 'SOURCE_ACTION_UNKNOWN' && src.code === 'SOURCE_ACTION_UNKNOWN') {
    return { expectedMatch: true, knownLimit: null, note: 'SOURCE_ACTION_UNKNOWN selector is distinct from OPERATION_BINDING' };
  }
  if (pattern.sourceLimitationTests?.probeExpectedStatus && src.status === 'Rejected' && src.code === 'OPERATION_BINDING') {
    return { expectedMatch: true, knownLimit: null, note: 'OPERATION_BINDING unsupported source check' };
  }
  if (src.status === 'skipped-incomplete-input' || src.status === 'PROFILE_UNSUPPORTED' || src.status === 'missing-source') {
    return { expectedMatch: false, knownLimit: null, note: 'unknown source skip is not success' };
  }
  const env = envelopesEqual(src, expected);
  if (env.compared) return { expectedMatch: env.match, knownLimit: null };
  if (expected && (expected.status || expected.code)) {
    const statusOk = !expected.status || src.status === expected.status;
    const codeOk = !expected.code || src.code === expected.code;
    return { expectedMatch: statusOk && codeOk, knownLimit: null };
  }
  return { expectedMatch: null, knownLimit: null };
}

export function runDesignerVerifiers(designers) {
  const dirs = readdirSync(designers, { withFileTypes: true }).filter(d => d.isDirectory()).map(d => join(designers, d.name)).sort();
  const rows = [];
  let ok = true;
  for (const dir of dirs) {
    const script = join(dir, 'verify.mjs');
    if (!existsSync(script)) {
      rows.push({ designer: basename(dir), ok: false, exit_code: null, error: 'missing verify.mjs' });
      ok = false;
      continue;
    }
    const result = spawnSync(process.execPath, [script], { cwd: dir, encoding: 'utf8', maxBuffer: 20_000_000 });
    const row = {
      designer: basename(dir),
      command: `node ${script}`,
      exit_code: result.status,
      verifier_sha256: sha256File(script),
      stdout: result.stdout || '',
      stderr: result.stderr || '',
      ok: result.status === 0,
    };
    if (result.status !== 0) ok = false;
    rows.push(row);
  }
  return { ok, verifiers: rows };
}

function loadInventory(designers) {
  const root = dirname(designers);
  const inventoryPath = join(root, 'PATTERN-INVENTORY.json');
  const suitePath = join(root, 'SOURCE-SUITE-RESULTS.json');
  return {
    inventoryPath: existsSync(inventoryPath) ? inventoryPath : null,
    inventory: existsSync(inventoryPath) ? readJson(inventoryPath) : null,
    suitePath: existsSync(suitePath) ? suitePath : null,
    suite: existsSync(suitePath) ? readJson(suitePath) : null,
  };
}

export function validateDesigners({ designers, manifest, compileExample = true, outDir }) {
  if (!designers) throw Object.assign(new Error('DESIGNERS_PATH_REQUIRED'), { code: 'DESIGNERS_PATH_REQUIRED' });
  const root = designers;
  const provenance = loadInventory(root);
  const verifierRun = runDesignerVerifiers(root);
  const dirs = readdirSync(root, { withFileTypes: true }).filter(d => d.isDirectory()).map(d => join(root, d.name)).sort();
  const byDesigner = new Map();
  for (const dir of dirs) {
    const casesPath = join(dir, 'cases.json');
    const cases = existsSync(casesPath) ? readJson(casesPath) : {};
    byDesigner.set(basename(dir), { dir, cases, patterns: collectPatterns(dir, cases), controls: cases.controls || [] });
  }

  let compiled = null;
  if (compileExample) {
    compiled = compilePublic({
      source: join(new URL('../examples/revision-step.mori', import.meta.url).pathname),
      manifest,
    });
  }

  const inventory = Array.isArray(provenance.inventory) ? provenance.inventory : [];
  const rows = [];
  const limitations = [];
  const unexpected = [];

  function addPatternRow(designer, pattern, linkedCases = []) {
    const dir = join(root, designer);
    const pack = byDesigner.get(designer);
    const verifier = verifierRun.verifiers.find(v => v.designer === designer);
    const nested = nestedSourceDescriptor(pattern, pack);
    const sources = resolveSource(dir, pattern, pack);
    if (!sources.length && pattern.sourceLimitationTests?.probe) {
      const probe = join(dir, pattern.sourceLimitationTests.probe);
      if (existsSync(probe)) sources.push(probe);
    }
    if (!sources.length && linkedCases.length) {
      for (const c of linkedCases) {
        for (const p of resolveSource(dir, c, pack)) if (!sources.includes(p)) sources.push(p);
      }
    }
    const probe = pattern.sourceLimitationTests?.probe ? join(dir, pattern.sourceLimitationTests.probe) : null;
    const files = [...sources];
    if (probe && existsSync(probe) && !files.includes(probe)) files.push(probe);
    if (!files.length) {
      rows.push({
        designer,
        id: pattern.id,
        name: pattern.name || pattern.id,
        linkedCases: linkedCases.map(c => c.id).filter(Boolean),
        source: { tier: 'source', status: 'missing-source', note: 'unknown source skip is not success' },
        UPLC: { tier: 'UPLC', status: 'not-run' },
        native: { tier: 'native', status: 'pending-unsupported', driver: 'parent-yaci-lucid' },
        private: { tier: 'private', status: 'unsupported' },
      });
      unexpected.push({ designer, id: pattern.id, reason: 'missing-source' });
      return;
    }
    if (verifier && verifier.ok === false) {
      unexpected.push({ designer, id: pattern.id, reason: 'verifier-failed', exit_code: verifier.exit_code });
    }
    for (const c of linkedCases) {
      if (KNOWN_LIMITS.has(c.id)) {
        limitations.push({
          id: c.id,
          pattern: pattern.id,
          known: KNOWN_LIMITS.get(c.id),
          note: 'named designer verify.mjs limitation; frozen cases.json FundedExpressionPrepared is obsolete',
        });
      }
    }
    for (const sourcePath of files) {
      const caseForFile = linkedCases.find(c => resolveSource(dir, c, pack).includes(sourcePath)) || pattern;
      const casePattern = { ...pattern, ...caseForFile };
      if (nested) {
        if (!casePattern.source) {
          casePattern.source = { ...nested, action: casePattern.sourceLimitationTests?.actionNames ? undefined : nested.action };
        }
        if (casePattern.ordinary == null && nested.ordinary) casePattern.ordinary = nested.ordinary;
        if (casePattern.financial == null && nested.financial) casePattern.financial = nested.financial;
      }
      if (casePattern.sourceLimitationTests?.actionNames) {
        casePattern.action = undefined;
        if (casePattern.source && typeof casePattern.source === 'object') {
          casePattern.source = { ...casePattern.source, action: undefined };
        }
      }
      const isProbeFile = Boolean(probe && sourcePath === probe);
      let src;
      if (pattern.sourceExpected || pattern.sourceLimitationTests || basename(sourcePath).startsWith('unsupported')) {
        src = runSourceCase(dir, casePattern, sourcePath);
        if (Array.isArray(pattern.sourceExpected) && Array.isArray(pattern.sources)) {
          const i = pattern.sources.indexOf(basename(sourcePath));
          if (i >= 0 && pattern.sourceExpected[i]) {
            src.expectedCode = pattern.sourceExpected[i];
            src.expectedMatch = src.code === src.expectedCode;
            if (src.expectedMatch === false) {
              unexpected.push({ designer, id: pattern.id, file: basename(sourcePath), actualCode: src.code, expectedCode: src.expectedCode });
            }
          }
        }
        if (isProbeFile) {
          if (src.status !== 'Rejected') {
            unexpected.push({ designer, id: pattern.id, file: basename(sourcePath), reason: 'probe-not-rejected', actual: src.status, code: src.code });
          }
        } else if (pattern.sourceLimitationTests?.expectedCode && src.code !== pattern.sourceLimitationTests.expectedCode) {
          unexpected.push({ designer, id: pattern.id, file: basename(sourcePath), actualCode: src.code, expectedCode: pattern.sourceLimitationTests.expectedCode });
        }
        if (!isProbeFile && pattern.sourceLimitationTests?.expectedCode === 'SOURCE_ACTION_UNKNOWN') {
          src.note = 'SOURCE_ACTION_UNKNOWN selector is distinct from OPERATION_BINDING';
        }
        if (src.code === 'OPERATION_BINDING') {
          src.note = 'OPERATION_BINDING unsupported source check (distinct from SOURCE_ACTION_UNKNOWN)';
        }
      } else {
        src = {
          tier: 'source',
          status: verifier?.ok ? 'verifier-passed' : 'verifier-failed',
          verifier_exit: verifier?.exit_code ?? null,
          note: 'fresh designer verify.mjs is the source oracle; frozen expected envelopes are not copied into outputs',
        };
      }
      const uplc = uplcStatus(sourcePath, manifest, compiled ? {
        ok: compiled.ok,
        programHash: compiled.metadata?.programHash,
        checkStatus: compiled.check?.status,
        positiveUplcValidation: compiled.positiveUplcValidation,
      } : null);
      rows.push({
        designer,
        id: pattern.id,
        name: pattern.name || pattern.id,
        file: basename(sourcePath),
        linkedCases: linkedCases.map(c => c.id).filter(Boolean),
        profile: detectProfile(readFileSync(sourcePath, 'utf8')),
        source: src,
        UPLC: uplc,
        native: { tier: 'native', status: pattern.nativeStatus || 'pending-unsupported', driver: 'parent-yaci-lucid', predicate: pattern.nativePredicate || null },
        private: { tier: 'private', status: 'unsupported', note: 'public experiment does not verify mandatory private/PCD claims' },
      });
      const known = pattern.limits || pattern.sourceLimitationTests || pattern.measuredGaps || pattern.nativeStatus;
      if (known) limitations.push({ id: pattern.id, file: basename(sourcePath), known });
      if (KNOWN_LIMITS.has(caseForFile.id)) {
        limitations.push({ id: caseForFile.id, file: basename(sourcePath), known: KNOWN_LIMITS.get(caseForFile.id) });
      }
    }
  }

  if (inventory.length) {
    for (const item of inventory) {
      const pack = byDesigner.get(item.designer);
      if (!pack) {
        unexpected.push({ designer: item.designer, id: item.id, reason: 'missing-designer' });
        rows.push({
          designer: item.designer,
          id: item.id,
          name: item.name,
          source: { tier: 'source', status: 'missing-source', note: 'unknown source skip is not success' },
          UPLC: { tier: 'UPLC', status: 'not-run' },
          native: { tier: 'native', status: 'pending-unsupported' },
          private: { tier: 'private', status: 'unsupported' },
        });
        continue;
      }
      const pattern = pack.patterns.find(p => p.id === item.id || p.name === item.name || p.name === item.id)
        || { id: item.id, name: item.name, designer: item.designer };
      const linked = [];
      if (Array.isArray(pack.cases.cases)) {
        for (const c of pack.cases.cases) {
          if (c.id === item.id || c.pattern === item.id || c.pattern === item.name || (c.id || '').startsWith(String(item.id).split('-')[0])) {
            linked.push(c);
          }
        }
      }
      if (pattern.relatedCases) linked.push(...pattern.relatedCases);
      if (pack.controls?.length && String(item.id).startsWith('AP')) {
        pattern.controls = pack.controls;
      }
      if (!linked.length && pattern.source) linked.push(pattern);
      addPatternRow(item.designer, { ...pattern, id: item.id, name: item.name || pattern.name }, [...new Map(linked.map(c => [c.id || c.name, c])).values()]);
    }
  } else {
    for (const [designer, pack] of byDesigner) {
      for (const pattern of pack.patterns) addPatternRow(designer, pattern, pattern.relatedCases || pack.cases.cases || []);
    }
  }

  const patternIds = inventory.length ? inventory.map(i => `${i.designer}:${i.id}`) : [...new Set(rows.map(r => `${r.designer}:${r.id}`))];
  const missingPatterns = inventory.length
    ? inventory.filter(i => !rows.some(r => r.designer === i.designer && r.id === i.id)).map(i => `${i.designer}:${i.id}`)
    : [];

  const compileFailed = compileExample && (!compiled || compiled.ok !== true);
  const ok = verifierRun.ok
    && !compileFailed
    && unexpected.length === 0
    && missingPatterns.length === 0
    && patternIds.length === (inventory.length || patternIds.length);

  const matrix = {
    schemaVersion: 'moriarty-eutxo-result-matrix/1',
    capability: 'public-atomic-experiment',
    production: false,
    privateVerification: 'explicitly-not-performed',
    nativeDriver: 'parent-isolated-yaci-lucid',
    nativeStatus: 'pending-unsupported',
    designers: root,
    provenance: {
      patternInventory: provenance.inventoryPath,
      sourceSuiteResults: provenance.suitePath,
      note: 'inventory/suite are input with provenance; fresh verifiers are the source oracle',
    },
    verifiers: verifierRun.verifiers.map(v => ({
      designer: v.designer,
      command: v.command,
      exit_code: v.exit_code,
      verifier_sha256: v.verifier_sha256,
      ok: v.ok,
      stdout: v.stdout,
    })),
    patternCount: patternIds.length,
    rows,
    limitations,
    compiledExample: compiled ? {
      ok: compiled.ok,
      programHash: compiled.metadata?.programHash,
      checkStatus: compiled.check?.status,
      buildStatus: compiled.build?.status,
      tests: compiled.check?.tests || [],
      positiveUplcValidation: compiled.positiveUplcValidation === true,
      artifacts: compiled.artifacts,
      blueprintPath: compiled.blueprintPath,
    } : null,
    discrepancies: unexpected,
    missingPatterns,
    ok,
  };
  if (outDir) {
    mkdirSync(outDir, { recursive: true });
    writeFileSync(join(outDir, 'matrix.json'), JSON.stringify(matrix, null, 2) + '\n');
  }
  return matrix;
}
