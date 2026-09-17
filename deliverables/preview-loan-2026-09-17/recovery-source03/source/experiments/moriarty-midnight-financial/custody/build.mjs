#!/usr/bin/env node
/** Compile generated custody wrappers with skip-zk. No installs, no proofs. */
import {spawnSync} from 'node:child_process';
import {mkdirSync, readFileSync, symlinkSync, writeFileSync, rmSync, existsSync, lstatSync} from 'node:fs';
import {dirname, join, resolve} from 'node:path';
import {fileURLToPath} from 'node:url';
import {asBuffer, generateWrappers, isDirectRun, sha256, validatePinnedInputs} from './generate.mjs';

const here = dirname(fileURLToPath(import.meta.url));
const worktree = resolve(here, '../../..');

function argValue(argv, name) {
  const i = argv.indexOf(name);
  if (i < 0 || !argv[i + 1]) throw new Error(`missing ${name}`);
  return argv[i + 1];
}

export function buildCustody(options = {}) {
  const hereDir = options.hereDir ?? here;
  const wt = options.worktree ?? worktree;
  const rf = options.readFileSync ?? readFileSync;
  const wf = options.writeFileSync ?? writeFileSync;
  const mk = options.mkdirSync ?? mkdirSync;
  const rm = options.rmSync ?? rmSync;
  const exists = options.existsSync ?? existsSync;
  const lstat = options.lstatSync ?? lstatSync;
  const symlink = options.symlinkSync ?? symlinkSync;
  const skipZk = options.skipZk ?? false;
  const skipToolchain = options.skipToolchain ?? false;
  const compile = options.compile;
  const argv = options.argv ?? process.argv;
  const cwd = options.cwd ?? process.cwd();
  const env = options.env ?? process.env;
  if (!skipZk) throw new Error('build requires --skip-zk');
  const outputDir = options.outputDir;
  if (!outputDir) throw new Error('missing --output-dir');
  const b = options.bindings ?? JSON.parse(asBuffer(rf(join(hereDir, 'bindings.json'))).toString('utf8'));
  const runtimeModules = b.toolchain.runtimeNodeModules;
  const commands = [];

  function run(command, opts = {}) {
    const result = spawnSync(command[0], command.slice(1), {
      cwd: opts.cwd ?? cwd,
      encoding: 'utf8',
      timeout: opts.timeout ?? 90000,
      env: opts.env ?? env,
    });
    commands.push({
      argv: command,
      cwd: opts.cwd ?? cwd,
      envNames: Object.keys(opts.env ?? env).sort(),
      exit: result.status,
      stdout: result.stdout,
      stderr: result.stderr,
      stdoutSha256: sha256(result.stdout ?? ''),
      stderrSha256: sha256(result.stderr ?? ''),
      encoding: 'utf8',
    });
    if (result.status !== 0) {
      throw new Error(`${command.join(' ')} exit ${result.status}\n${result.stderr || result.stdout}`);
    }
    return (result.stdout ?? '').trim();
  }

  const pinned = validatePinnedInputs({worktree: wt, bindings: b, readFileSync: rf});
  const generated = generateWrappers({worktree: wt, bindings: b, readFileSync: rf, pinned});
  const ownedLoan = asBuffer(rf(join(hereDir, 'loan.compact')));
  const ownedSwap = asBuffer(rf(join(hereDir, 'swap.compact')));
  if (!ownedLoan.equals(Buffer.from(generated.loanSource))) {
    throw new Error('stale wrapper loan.compact');
  }
  if (!ownedSwap.equals(Buffer.from(generated.swapSource))) {
    throw new Error('stale wrapper swap.compact');
  }

  if (compile) {
    compile('loan');
    compile('swap');
    return {status: 'checked', compilerInvocations: 2, skipToolchain: true};
  }

  if (!skipToolchain) {
    const nodeV = run([b.toolchain.node, '--version']);
    if (nodeV !== 'v' + b.toolchain.nodeVersion) throw new Error(`node ${nodeV}`);
    const compactV = run([b.toolchain.compact, '--version']);
    if (compactV !== 'compact ' + b.toolchain.compactWrapperVersion) throw new Error(`compact wrapper ${compactV}`);
    if (run([b.toolchain.compact, 'compile', '--version']) !== b.toolchain.compilerVersion) {
      throw new Error('compiler version');
    }
    if (run([b.toolchain.compact, 'compile', '--language-version']) !== b.toolchain.languageVersion) {
      throw new Error('language version');
    }
    if (run([b.toolchain.compact, 'compile', '--runtime-version']) !== b.toolchain.compactRuntimeVersion) {
      throw new Error('runtime version');
    }
    const runtimePkg = JSON.parse(asBuffer(rf(join(runtimeModules, '@midnight-ntwrk/compact-runtime/package.json'))).toString('utf8'));
    if (runtimePkg.version !== b.toolchain.compactRuntimeVersion) throw new Error('runtime package version');
  }

  mk(outputDir, {recursive: true});
  const artifacts = [];
  for (const [name, wrapper, kernel] of [
    ['loan', ownedLoan, pinned.loanKernel],
    ['swap', ownedSwap, pinned.swapKernel],
  ]) {
    const stage = join(outputDir, 'stage', name);
    const target = join(outputDir, name);
    rm(stage, {recursive: true, force: true});
    rm(target, {recursive: true, force: true});
    mk(stage, {recursive: true});
    wf(join(stage, 'arithmetic.compact'), pinned.arithmetic.bytes);
    wf(join(stage, 'kernel.compact'), kernel.bytes);
    wf(join(stage, `${name}.compact`), wrapper);
    if (sha256(asBuffer(rf(join(stage, 'arithmetic.compact')))) !== pinned.arithmetic.digest) throw new Error('arithmetic copy');
    if (sha256(asBuffer(rf(join(stage, 'kernel.compact')))) !== kernel.digest) throw new Error(`${name} kernel copy`);
    run([
      b.toolchain.compact, 'compile', '--skip-zk', '--compact-path', stage,
      join(stage, `${name}.compact`), target,
    ], {timeout: 90000});
    const nm = join(target, 'node_modules');
    if (exists(nm) || lstat(nm, {throwIfNoEntry: false})?.isSymbolicLink()) rm(nm, {recursive: true, force: true});
    symlink(runtimeModules, nm, 'dir');
    artifacts.push({name, target, kernel: kernel.digest, wrapper: sha256(wrapper)});
  }

  const nodeV = commands.find((c) => c.argv[1] === '--version' && String(c.argv[0]).endsWith('node'))?.stdout?.trim();
  const compactV = commands.find((c) => c.argv[1] === '--version' && String(c.argv[0]).includes('compact'))?.stdout?.trim();
  const receipt = {
    schema: 'moriarty.custody-build/1',
    argv,
    cwd,
    envNames: Object.keys(env).sort(),
    skipZk: true,
    keysGenerated: false,
    proofsGenerated: false,
    versions: {
      node: nodeV,
      compactWrapper: compactV,
      compiler: b.toolchain.compilerVersion,
      language: b.toolchain.languageVersion,
      runtime: b.toolchain.compactRuntimeVersion,
    },
    artifacts,
    commands: commands.map((c) => ({
      argv: c.argv,
      cwd: c.cwd,
      envNames: c.envNames,
      exit: c.exit,
      stdoutSha256: c.stdoutSha256,
      stderrSha256: c.stderrSha256,
      encoding: c.encoding,
      stdout: c.stdout,
      stderr: c.stderr,
    })),
  };
  wf(join(outputDir, 'build-receipt.json'), JSON.stringify(receipt, null, 2) + '\n');
  process.stdout.write(JSON.stringify({status: 'built', artifacts, commandCount: commands.length}) + '\n');
  return {status: 'built', artifacts, commandCount: commands.length};
}

export function runBuildCli(options = {}) {
  const argv = options.argv ?? process.argv;
  if (!argv.includes('--skip-zk')) throw new Error('build requires --skip-zk');
  return buildCustody({
    ...options,
    argv,
    outputDir: options.outputDir ?? resolve(argValue(argv, '--output-dir')),
    skipZk: true,
  });
}

if (isDirectRun(import.meta.url)) {
  runBuildCli();
}
