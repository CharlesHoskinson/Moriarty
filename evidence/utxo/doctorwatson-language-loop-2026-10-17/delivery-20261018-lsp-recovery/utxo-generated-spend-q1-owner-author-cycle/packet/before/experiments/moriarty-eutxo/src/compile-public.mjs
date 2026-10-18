import { readFileSync, writeFileSync, mkdirSync, rmSync, cpSync, existsSync, readdirSync, mkdtempSync } from 'node:fs';
import { dirname, join, resolve, basename } from 'node:path';
import { fileURLToPath } from 'node:url';
import { tmpdir } from 'node:os';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { compile } from '../../moriarty-language/src/frontend.ts';
import { admitPublicAtomic } from './admit.mjs';
import { lowerAiken } from './lower-aiken.mjs';

export const AIKEN = '/home/charl/.aiken/bin/aiken';
const here = dirname(fileURLToPath(import.meta.url));
export const ROOT = resolve(here, '..');
export const PROJECT = join(ROOT, 'eutxo-public');
export const BOUNDS = readFileSync(new URL('../../moriarty-language/spec/bounds.json', import.meta.url), 'utf8');

function sha256(buf) {
  return createHash('sha256').update(buf).digest('hex');
}

function aikenEnv() {
  return { ...process.env, PATH: `${dirname(AIKEN)}:${process.env.PATH || ''}` };
}

export function runAikenViaScript(args, cwd = PROJECT) {
  const command = [AIKEN, ...args].map(a => `'${String(a).replace(/'/g, `'\\''`)}'`).join(' ');
  const tty = spawnSync('script', ['-q', '-e', '-c', command, '/dev/null'], {
    cwd, encoding: 'utf8', env: aikenEnv(), maxBuffer: 20_000_000,
  });
  return { status: tty.status, stdout: tty.stdout || '', stderr: tty.stderr || '' };
}

export function runAiken(args, cwd = PROJECT) {
  const viaScript = runAikenViaScript(args, cwd);
  if (viaScript.status !== 0 || (viaScript.stdout || viaScript.stderr)) {
    return { ...viaScript, via: 'script' };
  }
  const direct = spawnSync(AIKEN, args, {
    cwd, encoding: 'utf8', env: aikenEnv(), maxBuffer: 20_000_000,
  });
  return { status: direct.status, stdout: direct.stdout || '', stderr: direct.stderr || '', via: 'direct' };
}

function stripAnsi(s) {
  return String(s || '').replace(/\x1b\[[0-9;]*m/g, '');
}

function parseAikenTtyTests(text) {
  const plain = stripAnsi(text);
  const tests = [];
  const re = /\b(PASS|FAIL)\b[^\n]*?\[mem:[^\]]+\]\s+([A-Za-z0-9_]+)/g;
  let m;
  while ((m = re.exec(plain))) {
    tests.push({ title: m[2], status: m[1] === 'PASS' ? 'success' : 'failure', kind: 'unit' });
  }
  return tests;
}

export function parseAikenCheckJson(stdout, stderr) {
  const text = `${stdout || ''}\n${stderr || ''}`;
  const plain = stripAnsi(text);
  const ttyTests = parseAikenTtyTests(plain);
  if (ttyTests.length) {
    const errors = [];
    if (/Summary \d+ error/.test(plain) && !/Summary 0 error/.test(plain) && !/0 errors/.test(plain)) {
      errors.push('AIKEN_ERROR');
    }
    return { ok: errors.length === 0, tests: ttyTests, summary: null, json: null, errors, raw: text };
  }
  const start = plain.indexOf('{');
  const end = plain.lastIndexOf('}');
  if (start >= 0 && end > start) {
    try {
      const json = JSON.parse(plain.slice(start, end + 1));
      if (json.modules || json.summary) {
        const tests = [];
        for (const mod of json.modules || []) {
          for (const t of mod.test || []) {
            tests.push({
              title: t.title,
              status: t.status,
              kind: t.kind,
              module: mod.name,
            });
          }
        }
        return { ok: true, tests, summary: json.summary || null, json, errors: [], raw: text };
      }
    } catch {
      // not JSON
    }
  }
  const errors = [];
  if (/Error /.test(plain) || /Summary \d+ error/.test(plain)) errors.push('AIKEN_ERROR');
  return {
    ok: false,
    tests: [],
    summary: null,
    json: null,
    errors: errors.length ? errors : ['AIKEN_JSON_MISSING'],
    raw: text,
  };
}

function copyIsolatedProject() {
  const dir = mkdtempSync(join(tmpdir(), 'moriarty-eutxo-'));
  cpSync(PROJECT, dir, {
    recursive: true,
    filter: (src) => {
      const base = basename(src);
      if (base === 'plutus.json') return false;
      if (base.endsWith('.uplc')) return false;
      if (src.includes(`${PROJECT}/artifacts`)) return false;
      if (src.includes(`${PROJECT}/validators`) && base.endsWith('.ak') && base !== 'placeholder.ak') return false;
      return true;
    },
  });
  mkdirSync(join(dir, 'validators'), { recursive: true });
  const stale = join(dir, 'plutus.json');
  if (existsSync(stale)) rmSync(stale);
  const artifacts = join(dir, 'artifacts');
  if (existsSync(artifacts)) rmSync(artifacts, { recursive: true, force: true });
  return dir;
}

function matchExpectedTests(parsed, expectedTests) {
  if (!expectedTests?.length) return { missing: [], unexpectedFail: [], titles: parsed.tests.map(t => t.title) };
  const missing = [];
  const unexpectedFail = [];
  for (const exp of expectedTests) {
    const found = parsed.tests.find(t => t.title === exp.title || t.title.endsWith(`.${exp.title}`) || t.title.endsWith(exp.title));
    if (!found) missing.push(exp.title);
    else if (found.status !== 'success' && found.status !== exp.status) unexpectedFail.push(found);
  }
  return { missing, unexpectedFail, titles: parsed.tests.map(t => t.title) };
}

export function compilePublic({ source, manifest, outDir = join(ROOT, 'artifacts') }) {
  const sourceText = typeof source === 'string' && source.includes('agreement') ? source : readFileSync(source, 'utf8');
  const man = typeof manifest === 'string' && !manifest.trim().startsWith('{')
    ? JSON.parse(readFileSync(manifest, 'utf8'))
    : typeof manifest === 'string' ? JSON.parse(manifest) : manifest;
  mkdirSync(outDir, { recursive: true });
  const publishedBlueprint = join(outDir, 'plutus.json');
  if (existsSync(publishedBlueprint)) rmSync(publishedBlueprint);
  const bound = compile(sourceText, BOUNDS).bound;
  const admitted = admitPublicAtomic(bound, man);
  if (!admitted.ok) {
    return { ok: false, admission: admitted, blueprintPath: null, positiveUplcValidation: false };
  }
  const lowered = lowerAiken(admitted);
  const isolated = copyIsolatedProject();
  try {
    writeFileSync(join(isolated, 'validators/atomic.ak'), lowered.validator);
    const check = runAiken(['check'], isolated);
    const parsed = parseAikenCheckJson(check.stdout, check.stderr);
    const testMatch = matchExpectedTests(parsed, lowered.expectedTests);
    let build = { status: 1, stdout: '', stderr: 'skipped: aiken check failed' };
    if (check.status === 0 && parsed.ok && testMatch.missing.length === 0 && testMatch.unexpectedFail.length === 0) {
      const staleIso = join(isolated, 'plutus.json');
      if (existsSync(staleIso)) rmSync(staleIso);
      build = runAiken(['build', '--uplc'], isolated);
    } else if (check.status === 0 && (!parsed.ok || testMatch.missing.length || testMatch.unexpectedFail.length)) {
      check.status = 1;
      check.stderr = `${check.stderr || ''}\nAIKEN_TEST_SUMMARY_INCOMPLETE missing=${JSON.stringify(testMatch.missing)} unexpected=${JSON.stringify(testMatch.unexpectedFail)}`;
    }
    const blueprintSrc = join(isolated, 'plutus.json');
    if (check.status === 0 && build.status === 0 && !existsSync(blueprintSrc)) {
      build.status = 1;
      build.stderr = `${build.stderr || ''}\nmissing plutus.json after aiken build`;
    }
    const toolchain = spawnSync(AIKEN, ['--version'], { encoding: 'utf8', env: aikenEnv() });
    const hashes = {
      sourceHash: bound.manifest.sourceHash,
      programHash: bound.programHash,
      boundsHash: bound.manifest.bounds.boundsHash,
      manifestHash: sha256(JSON.stringify(man)),
      validatorHash: sha256(lowered.validator),
      toolchain: (toolchain.stdout || '').trim(),
    };
    writeFileSync(join(outDir, 'atomic.ak'), lowered.validator);
    writeFileSync(join(outDir, 'sample-datum.json'), JSON.stringify(lowered.sampleDatum, null, 2) + '\n');
    writeFileSync(join(outDir, 'sample-redeemer.json'), JSON.stringify(lowered.sampleRedeemer, null, 2) + '\n');
    writeFileSync(join(outDir, 'data-schemas.json'), JSON.stringify(lowered.schemas, null, 2) + '\n');
    writeFileSync(join(outDir, 'admission.json'), JSON.stringify({
      ok: true,
      capability: admitted.capability,
      production: false,
      privateVerification: admitted.privateVerification,
      unsupportedPrivateClaims: admitted.unsupportedPrivateClaims,
      timePredicate: admitted.timePredicate,
      timePredicateDoc: admitted.timePredicateDoc,
      nativePredicate: admitted.nativePredicate,
      nativeSubset: admitted.nativeSubset,
      genesisPrecondition: admitted.genesisPrecondition,
      positiveUplcValidation: lowered.positiveUplcValidation === true,
      hashes,
    }, null, 2) + '\n');
    const ok = check.status === 0 && build.status === 0 && existsSync(blueprintSrc);
    if (ok) {
      cpSync(blueprintSrc, publishedBlueprint);
      const uplcDir = join(isolated, 'artifacts');
      if (existsSync(uplcDir)) {
        mkdirSync(join(outDir), { recursive: true });
        for (const name of readdirSync(uplcDir).filter(n => n.endsWith('.uplc'))) {
          cpSync(join(uplcDir, name), join(outDir, name));
        }
      }
      writeFileSync(join(PROJECT, 'validators/atomic.ak'), lowered.validator);
      cpSync(publishedBlueprint, join(PROJECT, 'plutus.json'));
    }
    return {
      ok,
      admission: admitted,
      metadata: { ...lowered.metadata, hashes, positiveUplcValidation: lowered.positiveUplcValidation === true },
      check: {
        status: check.status,
        stdout: check.stdout,
        stderr: check.stderr,
        tests: parsed.tests,
        summary: parsed.summary,
        parseOk: parsed.ok,
        parseErrors: parsed.errors,
        missingTests: testMatch.missing,
        unexpectedFail: testMatch.unexpectedFail,
      },
      build: { status: build.status, stdout: build.stdout, stderr: build.stderr },
      artifacts: outDir,
      validatorPath: join(outDir, 'atomic.ak'),
      blueprintPath: ok && existsSync(publishedBlueprint) ? publishedBlueprint : null,
      positiveUplcValidation: ok && lowered.positiveUplcValidation === true,
    };
  } finally {
    rmSync(isolated, { recursive: true, force: true });
  }
}
