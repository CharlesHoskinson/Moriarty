#!/usr/bin/env node
/** Proven-custody builder source. Compiler remains disabled without a later admission. */
import {existsSync, lstatSync, mkdirSync, readFileSync, rmSync, writeFileSync} from 'node:fs';
import {dirname, join, relative, resolve, sep} from 'node:path';
import {fileURLToPath} from 'node:url';
import {asBuffer, generateWrappers, isDirectRun, sha256, validatePinnedInputs} from '../custody/generate.mjs';

const here = dirname(fileURLToPath(import.meta.url));

function insideRoot(root, target, lstat = lstatSync) {
  const r = resolve(root);
  const t = resolve(target);
  const rel = relative(r, t);
  if (rel.startsWith('..') || rel === '') {
    if (t !== r && (rel === '' ? false : rel.startsWith('..'))) throw new Error('output escapes build root');
    if (t !== r && !t.startsWith(r.endsWith(sep) ? r : r + sep)) throw new Error('output escapes build root');
  }
  if (rel.startsWith('..')) throw new Error('output escapes build root');
  let cur = t;
  for (let i = 0; i < 64; i += 1) {
    const st = lstat(cur, {throwIfNoEntry: false});
    if (st?.isSymbolicLink()) throw new Error('symlink output escape');
    const parent = dirname(cur);
    if (parent === cur) break;
    if (cur === r) break;
    cur = parent;
  }
  return t;
}

function compileArgv(compact, stage, name, target) {
  return [compact, 'compile', '--compact-path', stage, join(stage, `${name}.compact`), target];
}

export async function buildProvenCustody(options = {}) {
  const rf = options.readFileSync ?? readFileSync;
  const wf = options.writeFileSync ?? writeFileSync;
  const mk = options.mkdirSync ?? mkdirSync;
  const rm = options.rmSync ?? rmSync;
  const exists = options.existsSync ?? existsSync;
  const lstat = options.lstatSync ?? lstatSync;
  const worktree = options.worktree ?? resolve(here, '../../..');
  const custodyDir = options.custodyDir ?? resolve(here, '../custody');
  const bindings = options.bindings ?? JSON.parse(asBuffer(rf(join(custodyDir, 'bindings.json'))).toString('utf8'));
  if (!options.outputDir) throw new Error('missing outputDir');
  if (!options.buildRoot) throw new Error('missing buildRoot');
  const outputDir = insideRoot(options.buildRoot, options.outputDir, lstat);
  const pinned = validatePinnedInputs({worktree, bindings, readFileSync: rf});
  const generated = generateWrappers({worktree, bindings, readFileSync: rf, pinned});
  const ownedLoan = asBuffer(rf(join(custodyDir, 'loan.compact')));
  const ownedSwap = asBuffer(rf(join(custodyDir, 'swap.compact')));
  if (!ownedLoan.equals(Buffer.from(generated.loanSource))) throw new Error('stale wrapper loan.compact');
  if (!ownedSwap.equals(Buffer.from(generated.swapSource))) throw new Error('stale wrapper swap.compact');
  const compact = bindings.toolchain.compact;
  const invocations = [];
  for (const name of ['loan', 'swap']) {
    const stage = join(outputDir, 'stage', name);
    const target = join(outputDir, name);
    invocations.push({name, argv: compileArgv(compact, stage, name, target)});
  }
  if (options.processAdapter) {
    mk(outputDir, {recursive: true});
    const results = [];
    for (const inv of invocations) {
      const stage = join(outputDir, 'stage', inv.name);
      mk(stage, {recursive: true});
      const kernel = inv.name === 'loan' ? pinned.loanKernel : pinned.swapKernel;
      const wrapper = inv.name === 'loan' ? ownedLoan : ownedSwap;
      wf(join(stage, 'arithmetic.compact'), pinned.arithmetic.bytes);
      wf(join(stage, 'kernel.compact'), kernel.bytes);
      wf(join(stage, `${inv.name}.compact`), wrapper);
      if (inv.argv.includes('--skip-zk')) throw new Error('skip-zk is not a proven compile');
      results.push(options.processAdapter(inv.argv));
    }
    return {
      status: 'adapter-checked',
      proven: false,
      compilerInvoked: false,
      manifestKind: 'unproven-test-manifest',
      label: 'unproven-test-manifest',
      invocations,
      adapterResults: results,
    };
  }
  return {
    status: 'source-checked',
    proven: false,
    compilerInvoked: false,
    manifestKind: 'unproven-source-check',
    label: 'unproven',
    compilerDisabled: true,
    wrapperHashes: {
      loan: sha256(ownedLoan),
      swap: sha256(ownedSwap),
    },
    invocations,
    note: 'Full compiler remains disabled until a reviewed full-build admission inspects actual compiler and proof assets.',
  };
}

export function runBuildCli(argv = process.argv) {
  if (argv.includes('--help') || argv.includes('--explain')) {
    process.stdout.write('build-proven: source check only. Full compile requires a later reviewed admission. Do not pass --skip-zk.\n');
    return 0;
  }
  throw new Error('CLI compile is disabled without full-build admission');
}

if (isDirectRun(import.meta.url)) {
  runBuildCli();
}
