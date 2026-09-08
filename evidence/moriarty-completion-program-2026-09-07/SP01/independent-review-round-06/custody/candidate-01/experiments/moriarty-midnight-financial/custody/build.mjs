#!/usr/bin/env node
/** Compile generated custody wrappers with skip-zk. No installs, no proofs. */
import {spawnSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {cpSync, mkdirSync, readFileSync, symlinkSync, writeFileSync, rmSync, existsSync, lstatSync} from 'node:fs';
import {dirname, join, resolve} from 'node:path';
import {fileURLToPath} from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));
const worktree = resolve(here, '../../..');
const bindings = JSON.parse(readFileSync(join(here, 'bindings.json'), 'utf8'));

function argValue(name) {
  const i = process.argv.indexOf(name);
  if (i < 0 || !process.argv[i + 1]) throw new Error(`missing ${name}`);
  return process.argv[i + 1];
}
if (!process.argv.includes('--skip-zk')) throw new Error('build requires --skip-zk');
const outputDir = resolve(argValue('--output-dir'));
const runtimeModules = bindings.toolchain.runtimeNodeModules;
const commands = [];

function sha256(buf) {
  return createHash('sha256').update(buf).digest('hex');
}
function run(command, opts = {}) {
  const result = spawnSync(command[0], command.slice(1), {
    cwd: opts.cwd ?? process.cwd(),
    encoding: 'utf8',
    timeout: opts.timeout ?? 90000,
    env: opts.env ?? process.env,
  });
  commands.push({
    argv: command,
    cwd: opts.cwd ?? process.cwd(),
    envNames: Object.keys(opts.env ?? process.env).sort(),
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

const nodeV = run([bindings.toolchain.node, '--version']);
if (nodeV !== 'v' + bindings.toolchain.nodeVersion) throw new Error(`node ${nodeV}`);
const compactV = run([bindings.toolchain.compact, '--version']);
if (compactV !== 'compact ' + bindings.toolchain.compactWrapperVersion) throw new Error(`compact wrapper ${compactV}`);
if (run([bindings.toolchain.compact, 'compile', '--version']) !== bindings.toolchain.compilerVersion) {
  throw new Error('compiler version');
}
if (run([bindings.toolchain.compact, 'compile', '--language-version']) !== bindings.toolchain.languageVersion) {
  throw new Error('language version');
}
if (run([bindings.toolchain.compact, 'compile', '--runtime-version']) !== bindings.toolchain.compactRuntimeVersion) {
  throw new Error('runtime version');
}
const runtimePkg = JSON.parse(readFileSync(join(runtimeModules, '@midnight-ntwrk/compact-runtime/package.json'), 'utf8'));
if (runtimePkg.version !== bindings.toolchain.compactRuntimeVersion) throw new Error('runtime package version');

function pin(rel, expected) {
  const abs = join(worktree, rel);
  const bytes = readFileSync(abs);
  const digest = sha256(bytes);
  if (digest !== expected) throw new Error(`stale ${rel}: ${digest}`);
  return {abs, bytes, digest};
}
const arithmetic = pin('experiments/moriarty-language/compact/arithmetic.compact', bindings.digests.arithmetic);
const loanKernel = pin('experiments/moriarty-language/compact/generated/loan/kernel.compact', bindings.digests.loanKernel);
const swapKernel = pin('experiments/moriarty-language/compact/generated/swap/kernel.compact', bindings.digests.swapKernel);
const loanWrapper = readFileSync(join(here, 'loan.compact'));
const swapWrapper = readFileSync(join(here, 'swap.compact'));
if (!loanWrapper.includes(bindings.digests.loanProgram)) throw new Error('loan wrapper missing program digest');
if (!swapWrapper.includes(bindings.digests.swapProgram)) throw new Error('swap wrapper missing program digest');
if (!loanWrapper.includes(bindings.digests.loanKernel)) throw new Error('loan wrapper missing kernel digest');
if (!swapWrapper.includes(bindings.digests.swapKernel)) throw new Error('swap wrapper missing kernel digest');
if (!loanWrapper.includes('transition0(') || !loanWrapper.includes('transition1(')) throw new Error('loan wrapper must invoke kernel');
if (!swapWrapper.includes('transition0(') || !swapWrapper.includes('transition1(')) throw new Error('swap wrapper must invoke kernel');

mkdirSync(outputDir, {recursive: true});
const artifacts = [];
for (const [name, wrapper, kernel] of [
  ['loan', loanWrapper, loanKernel],
  ['swap', swapWrapper, swapKernel],
]) {
  const stage = join(outputDir, 'stage', name);
  const target = join(outputDir, name);
  rmSync(stage, {recursive: true, force: true});
  rmSync(target, {recursive: true, force: true});
  mkdirSync(stage, {recursive: true});
  writeFileSync(join(stage, 'arithmetic.compact'), arithmetic.bytes);
  writeFileSync(join(stage, 'kernel.compact'), kernel.bytes);
  writeFileSync(join(stage, `${name}.compact`), wrapper);
  if (sha256(readFileSync(join(stage, 'arithmetic.compact'))) !== arithmetic.digest) throw new Error('arithmetic copy');
  if (sha256(readFileSync(join(stage, 'kernel.compact'))) !== kernel.digest) throw new Error(`${name} kernel copy`);
  run([
    bindings.toolchain.compact, 'compile', '--skip-zk', '--compact-path', stage,
    join(stage, `${name}.compact`), target,
  ], {timeout: 90000});
  const nm = join(target, 'node_modules');
  if (existsSync(nm) || lstatSync(nm, {throwIfNoEntry: false})?.isSymbolicLink()) rmSync(nm, {recursive: true, force: true});
  symlinkSync(runtimeModules, nm, 'dir');
  artifacts.push({name, target, kernel: kernel.digest, wrapper: sha256(wrapper)});
}

const receipt = {
  schema: 'moriarty.custody-build/1',
  argv: process.argv,
  cwd: process.cwd(),
  envNames: Object.keys(process.env).sort(),
  skipZk: true,
  keysGenerated: false,
  proofsGenerated: false,
  versions: {
    node: nodeV,
    compactWrapper: compactV,
    compiler: bindings.toolchain.compilerVersion,
    language: bindings.toolchain.languageVersion,
    runtime: bindings.toolchain.compactRuntimeVersion,
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
writeFileSync(join(outputDir, 'build-receipt.json'), JSON.stringify(receipt, null, 2) + '\n');
process.stdout.write(JSON.stringify({status: 'built', artifacts, commandCount: commands.length}) + '\n');
