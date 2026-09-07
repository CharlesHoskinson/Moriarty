# Candidate A compiler phase diagnostic Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Obtain a bounded observation of the public compiler stages in one new original-pilot invocation, without changing its semantic inputs or promoting the diagnostic to H1 evidence.

**Architecture:** A CommonJS observer replaces six in-memory export bindings before loading the unchanged CLI. A separate Python recorder owns process groups, source/runtime checks, original streams, trace validation, and exclusive receipt paths. Mock controls and four tiny compilations precede independent root authorization of the single full invocation.

**Tech Stack:** Installed Quint 0.32.0, Node 24.18.1, the admitted Python virtual environment, GNU time, and the pinned RH002 process-group cleanup routines. No installations.

## Global Constraints

- Specification: `docs/superpowers/specs/2026-09-06-candidate-a-compiler-phase-diagnostic-design.md`, SHA-256 `19ff70c32004d824ee1b5e34e722425a354412d3548f321a943b6e32fc9ca67a`.
- Status: **specified-only**. This document supplies code for review; none of its controls or native invocations has run.
- Worktree: `/home/charl/Moriarty/.worktrees/s01-audit-start`.
- New executable files and outputs only under `.superpowers/sdd/a5-compiler-phase-diagnostic-20260906/`. Create the directory exclusively. No old receipt, model, runtime, helper, Foreman, or Git state mutation; no commits in this plan.
- Keep the 24-file compilation view, its two import additions, and all 30 frozen originals unchanged. The old timeout series stays closed; historical dispatch `43f33721da7a3bfb11dd1a91f1f548d5110ab93c` and runtime bootstrap `900bb2051225b4a3d99bf422c3b2e5e386e3e7bc` remain historical fields.
- Observe only `load`, `parse`, `typecheck`, `compile`, `outputCompilationTarget`, `outputResult`. Preserve each actual callback's receiver, argument identities, original call count, return/promise identity, and fulfillment/rejection values. Do not force Left-skipped callbacks.
- No observer signal/exit/global exception/global rejection handler, process.exit replacement, intermediate IR dump, additional compiler pipeline, or private compiler hook.
- Trace: at most 32 records and 16 KiB, separate from stdout/stderr. Observation failure reserves exit 74 with a constant bounded error; no phase admission on that outcome.
- Mock child budget: 15 seconds and 128 MiB Node heap, strict unhandled-rejection mode. Seven fresh mock processes. Tiny native child budget: 120 seconds and 4096-MiB Node/JVM settings, exactly four fresh processes. Full original: once, 900 seconds and 4096-MiB Node/JVM settings. Each budget starts immediately before Popen and includes process construction; cleanup allows at most 5 seconds per TERM/KILL wait. Pinning is outside native timing.
- The full command requires independently admitted controls, a terminal and original outer terminal for A4 fresh97 with no unresolved owned group, and a separate root dispatch record. No heavyweight overlap, retries, factored compile, solver, resource increase, or hypothesis adoption.
- Incremental tool transport records are written under `transport/`, outside recorder-finalized stage directories. Missing records remain missing. Root takeover is required after an interrupted handoff or failed finalization; do not invent a terminal/index or rerun native work.

## Task CP001: Create and independently review the observer sources

**Files:** Create `observer.cjs`, `launch.cjs`, `controls.cjs`, and `record.py` inside the dedicated directory. Their complete contents follow. The block immediately after each `file:` comment is the authoritative byte content, including its final newline.

**Interfaces:** `observer.cjs` exports `PHASES`, `fail()`, `createTrace(path, io, clock)`, and `install(exportsObject, emit)`. `launch.cjs` takes exactly one invocation JSON path. `controls.cjs` takes a case name and an exclusive existing output directory. `record.py` provides the finite preparation, control, root-adoption, and full modes listed below. No module import executes a native tool except the explicit launch entry point.

- [ ] Materialize the following exact files after root approves this plan. Do not execute them yet. The Python extraction command at CP002 creates only these four files from this document.

<!-- file: observer.cjs -->
```javascript
'use strict';
const fs = require('node:fs');
const either = require('/home/charl/.npm-global/lib/node_modules/@informalsystems/quint/node_modules/@sweet-monads/either/cjs/index.js');
const prototype = Object.getPrototypeOf(either.right(null));
const PHASES = Object.freeze(['load', 'parse', 'typecheck', 'compile', 'outputCompilationTarget', 'outputResult']);
function fail() {
  try { fs.writeSync(2, 'phase-observer: diagnostic failure\n'); } catch (_) {}
  process.exit(74);
}
function outcome(value) {
  try {
    if (Object.getPrototypeOf(value) !== prototype) fail();
    const d = Object.getOwnPropertyDescriptor(value, 'type');
    if (!d || !Object.hasOwn(d, 'value') || !['Left', 'Right'].includes(d.value)) fail();
    return d.value;
  } catch (_) { fail(); }
}
function createTrace(path, io = fs, clock = process.hrtime.bigint) {
  let fd;
  try { fd = io.openSync(path, 'wx', 0o600); } catch (_) { fail(); }
  let sequence = 0, bytes = 0, previous = -1n;
  return function emit(phase, event, result) {
    try {
      if (!PHASES.includes(phase) || !['enter', 'return', 'resolve', 'throw', 'reject'].includes(event)) fail();
      if (!['stage', 'Left', 'Right', 'undefined', 'thrown', 'rejected'].includes(result)) fail();
      const ns = clock();
      if (typeof ns !== 'bigint' || ns < 0n || ns < previous) fail();
      const raw = Buffer.from(JSON.stringify({seq: sequence + 1, phase, event, outcome: result, ns: String(ns)}) + '\n');
      if (sequence >= 32 || bytes + raw.length > 16384) fail();
      if (io.writeSync(fd, raw) !== raw.length) fail();
      sequence += 1; bytes += raw.length; previous = ns;
    } catch (_) { fail(); }
  };
}
function install(commands, emit) {
  const safeEmit = (...args) => { try { emit(...args); } catch (_) { fail(); } };
  try {
  for (const phase of PHASES) {
    const descriptor = Object.getOwnPropertyDescriptor(commands, phase);
    if (!descriptor || typeof descriptor.value !== 'function' || !descriptor.writable) fail();
    const original = descriptor.value;
    const observed = function (...args) {
      safeEmit(phase, 'enter', phase === 'outputResult' ? outcome(args[0]) : 'stage');
      let value;
      try { value = Reflect.apply(original, this, args); }
      catch (error) { safeEmit(phase, 'throw', 'thrown'); throw error; }
      if (value instanceof Promise) {
        // Return the original promise. Both observing callbacks return undefined;
        // neither forwards rejection into the otherwise unused observing promise.
        try {
          Promise.prototype.then.call(value,
            result => { safeEmit(phase, 'resolve', outcome(result)); },
            error => { safeEmit(phase, 'reject', 'rejected'); });
        } catch (_) { fail(); }
      } else {
        safeEmit(phase, 'return', phase === 'outputResult' && value === undefined ? 'undefined' : outcome(value));
      }
      return value;
    };
    Object.defineProperty(commands, phase, {...descriptor, value: observed});
  }
  } catch (_) { fail(); }
}
module.exports = {PHASES, fail, createTrace, install};
```

<!-- file: launch.cjs -->
```javascript
'use strict';
const fs = require('node:fs');
const path = require('node:path');
let hooks;
try { hooks = require('./observer.cjs'); }
catch (_) {
  try { fs.writeSync(2, 'phase-observer: diagnostic failure\n'); } catch (_) {}
  process.exit(74);
}
const {fail, createTrace, install} = hooks;
const CLI = '/home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/cli.js';
const NODE = '/home/charl/.foreman/tools/fnm/node-versions/v24.18.1/installation/bin/node';
const launched = process.argv.slice();
let spec;
try {
  if (launched.length !== 3 || process.execPath !== NODE) fail();
  spec = JSON.parse(fs.readFileSync(launched[2], 'utf8'));
  if (!Array.isArray(spec.effectiveArgv) || spec.effectiveArgv.length !== 8 ||
      spec.effectiveArgv[0] !== NODE || spec.effectiveArgv[1] !== CLI ||
      spec.effectiveArgv[2] !== 'compile' || spec.effectiveArgv[5] !== '--target=json' ||
      spec.effectiveArgv[7] !== '--verbosity=0') fail();
  if (!path.basename(launched[2]).endsWith('-invocation.json')) fail();
  const folder = path.join(path.dirname(launched[2]), path.basename(launched[2], '-invocation.json'));
  if (spec.trace !== path.join(folder, 'trace.jsonl') || spec.argvReceipt !== path.join(folder, 'argv.json')) fail();
  const raw = Buffer.from(JSON.stringify({actualLaunchArgv: launched, effectiveArgv: spec.effectiveArgv,
    execPath: process.execPath, execArgv: process.execArgv, cwd: process.cwd()}) + '\n');
  if (raw.length > 16384) fail();
  const fd = fs.openSync(spec.argvReceipt, 'wx', 0o600);
  if (fs.writeSync(fd, raw) !== raw.length) fail();
  fs.closeSync(fd);
} catch (_) { fail(); }
const emit = createTrace(spec.trace);
try {
  const commands = require('/home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/cliCommands.js');
  install(commands, emit);
  process.argv = spec.effectiveArgv.slice();
  require(CLI);
} catch (_) { fail(); }
```

<!-- file: controls.cjs -->
```javascript
'use strict';
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const {PHASES, createTrace, install} = require('./observer.cjs');
const {left, right} = require('/home/charl/.npm-global/lib/node_modules/@informalsystems/quint/node_modules/@sweet-monads/either/cjs/index.js');
const [kind, directory] = process.argv.slice(2);
const trace = path.join(directory, 'trace.jsonl');
function memoryTrace() {
  const rows = [];
  const emit = createTrace('memory', {openSync: () => 9, writeSync: (fd, raw) => {
    rows.push(JSON.parse(raw.toString())); return raw.length;
  }}, () => BigInt(rows.length));
  return {emit, rows};
}
function commands(fn) { return Object.fromEntries(PHASES.map(p => [p, fn])); }
async function identity() {
  const report = [];
  for (const mode of ['sync-right', 'sync-left', 'resolve', 'reject', 'throw']) {
    const {emit, rows} = memoryTrace();
    const receiver = {}, argument = {mutable: 0}, result = mode === 'sync-left' ? left(argument) : right(argument);
    const error = new Error('identity sentinel');
    let calls = 0, promise;
    const exports = commands(function (...args) {
      calls += 1; assert.equal(this, receiver); assert.equal(args.length, 1); assert.equal(args[0], argument);
      argument.mutable += 1;
      if (mode === 'throw') throw error;
      if (mode === 'resolve') return promise = Promise.resolve(result);
      if (mode === 'reject') return promise = Promise.reject(error);
      return result;
    });
    install(exports, emit);
    if (mode === 'throw') assert.throws(() => exports.load.call(receiver, argument), e => e === error);
    else {
      const returned = exports.load.call(receiver, argument);
      if (mode === 'resolve' || mode === 'reject') {
        assert.equal(returned, promise);
        if (mode === 'resolve') assert.equal(await returned, result);
        else await assert.rejects(returned, e => e === error);
      } else assert.equal(returned, result);
    }
    assert.equal(calls, 1); assert.equal(argument.mutable, 1);
    assert.equal(rows.length, 2); assert.equal(rows[0].event, 'enter');
    assert.equal(rows[1].event, ({resolve: 'resolve', reject: 'reject', throw: 'throw'})[mode] || 'return');
    assert.equal(rows[1].outcome, ({'sync-left': 'Left', reject: 'rejected', throw: 'thrown'})[mode] || 'Right');
    report.push({mode, calls, rows});
  }
  for (const broken of [false, true]) {
    const {emit, rows} = memoryTrace(), called = [], objects = [];
    const exports = Object.fromEntries(PHASES.map(phase => [phase, function (arg) {
      called.push(phase); objects.push(arg);
      if (phase === 'outputResult') return undefined;
      return Promise.resolve(phase === 'compile' && broken ? left(arg) : right(arg));
    }]));
    const state = {};
    install(exports, emit);
    // Same asyncChain shape as cli.js:59–60,119–124. No synthetic skipped call.
    await exports.load(state).then(r => r.asyncChain(exports.parse))
      .then(r => r.asyncChain(exports.typecheck)).then(r => r.asyncChain(exports.compile))
      .then(r => r.asyncChain(exports.outputCompilationTarget)).then(exports.outputResult);
    const wanted = PHASES.filter(p => !broken || p !== 'outputCompilationTarget');
    assert.deepEqual(called, wanted);
    assert.deepEqual(rows.filter(r => r.event === 'enter').map(r => r.phase), wanted);
    assert.ok(objects.slice(0, -1).every(v => v === state));
    assert.equal(objects.at(-1).value, state);
    assert.equal(rows.at(-2).outcome, broken ? 'Left' : 'Right');
    report.push({mode: broken ? 'left-chain' : 'right-chain', calls: called, rows});
  }
  // A whole event-loop turn under strict rejection handling catches a rejected
  // unused observer continuation; there is no global event handler in this test.
  await new Promise(resolve => setImmediate(resolve));
  process.stdout.write(JSON.stringify({ok: true, controls: report}) + '\n');
}
if (kind === 'identity') {
  identity().catch(error => { process.stderr.write(String(error) + '\n'); process.exitCode = 1; });
} else {
  let io = fs, clock = process.hrtime.bigint;
  if (kind === 'open') fs.mkdirSync(trace);
  if (kind === 'write') io = {openSync: fs.openSync, writeSync: () => { throw new Error('injected write'); }};
  if (kind === 'partial') io = {openSync: fs.openSync, writeSync: (fd, raw) => fs.writeSync(fd, raw.subarray(0, raw.length - 1))};
  if (kind === 'bytes') clock = () => BigInt('1'.repeat(17000));
  const emit = createTrace(trace, io, clock);
  if (kind === 'records') {
    for (let i = 0; i < 33; i++) emit('load', 'enter', 'stage');
  } else if (kind === 'classify') {
    const exports = commands(() => {
      fs.writeFileSync(path.join(directory, 'original-called.txt'), '1\n', {flag: 'wx'});
      return {};
    });
    install(exports, emit); exports.load({});
  } else if (['open', 'write', 'partial', 'bytes'].includes(kind)) {
    const exports = commands(() => {
      fs.writeFileSync(path.join(directory, 'original-called.txt'), '1\n', {flag: 'wx'});
      return right(null);
    });
    install(exports, emit); exports.load({});
  } else throw new Error('unknown control');
  throw new Error('fault failed to stop observer');
}
```

The production observer is non-async. CommonJS `cli.js` reads the same `cliCommands` object and uses its properties when constructing the chain; interception happens before `require(CLI)`. Result classification reads only the pinned Either instance's own `type` data property, never source/value contents. The observing promise handlers return undefined on both outcomes, so their unused derived promise does not reject on an original rejection. Logging failure exits immediately; a signal that preempts this remains an authentic signal outcome, never a manufactured 74.

The synthetic identity program deliberately supplies its own final catch only for its test function, not a process-global handler or a production CLI handler. Its mock outputResult returns normally to test ordinary return identity; the real baseline controls below test actual process.exit behavior without replacing it.

<!-- file: record.py -->
```python
"""Finite diagnostic recorder. Observation preservation is not compiler acceptance."""
import datetime
import hashlib
import json
import os
from pathlib import Path
import re
import runpy
import signal
import subprocess
import sys
import time

OUT = Path(__file__).resolve().parent
ROOT = OUT.parents[2]
PYTHON = '/home/charl/Moriarty/.venv/bin/python'
NODE = '/home/charl/.foreman/tools/fnm/node-versions/v24.18.1/installation/bin/node'
PACKAGE = Path('/home/charl/.npm-global/lib/node_modules/@informalsystems/quint')
CLI = str(PACKAGE / 'dist/src/cli.js')
PLAN = ROOT / 'docs/superpowers/plans/2026-09-06-candidate-a-compiler-phase-diagnostic.md'
DESIGN = ROOT / 'docs/superpowers/specs/2026-09-06-candidate-a-compiler-phase-diagnostic-design.md'
PLANNING = ROOT / 'evidence/s02-candidate-a-completion/a5/compile-phase-diagnostic-planning'
OLD = ROOT / '.superpowers/sdd/a5-factoring-receipts'
VIEW = OLD / 'pilot-compilation-view'
ALIAS = OLD / 'alias-visibility-control'
RUNTIME = OLD / 'tool-store/manifest.json'
RH = ROOT / 'evidence/s02-candidate-a-completion/a4/native-resources/runner.py'
RH_SHA = 'd8973d3541269d1be2ce524f2482e5f5b858f15f7d9117e3fe6d5cd2171d660d'
A4 = ROOT / '.superpowers/sdd/a4-task6-resumption-20260906'
A4_SHA = '63d8f00d024dbafb08fef0d48da96c91ca594bfe33476946ab0960bfd7dcf666'
PHASES = ['load', 'parse', 'typecheck', 'compile', 'outputCompilationTarget', 'outputResult']
MODES = ['prepare', 'mocks', 'adopt-mocks', 'aliases', 'adopt-aliases', 'authorize-full', 'full', 'intake']

def require(value, message):
    if not value:
        raise RuntimeError(message)

def sha(path):
    with Path(path).open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()

require(sys.flags.optimize == 0 and sys.flags.dont_write_bytecode, 'require optimization zero and -B')
require(sys.executable == PYTHON, 'exact admitted virtualenv interpreter required')
require(sha(A4 / 'recorder.py') == A4_SHA and sha(RH) == RH_SHA, 'admitted pure utilities changed')
# Import only; neither module's __main__ entry runs. No global variable overrides.
utility = runpy.run_path(str(A4 / 'recorder.py'))
group = runpy.run_path(str(RH))
pin, write, read, archive = (utility[n] for n in ('pin', 'write', 'read', 'archive'))
cleanup_group, group_exists = group['cleanup_group'], group['group_exists']
require(sha(A4 / 'recorder.py') == A4_SHA and sha(RH) == RH_SHA, 'utilities changed during import')

def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

def head():
    return subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip()

def expected_argv(kind):
    if kind == 'full':
        entry = VIEW / 'view/specs/quint/s02/factored_verification/candidate_a_funding_pilot.qnt'
        main, invariant = 'candidate_a_funding_pilot', 'pilotSafety'
    else:
        require(kind in ('success', 'error'), 'bad input kind')
        suffix = 'direct' if kind == 'success' else 'original'
        entry = ALIAS / 'src' / ('driver_' + suffix + '.qnt')
        main, invariant = 'alias_visibility_' + suffix, 'safety'
    return [NODE, CLI, 'compile', str(entry), '--main=' + main,
            '--target=json', '--invariant=' + invariant, '--verbosity=0']

def check_pins(rows):
    require(len({r['path'] for r in rows}) == len(rows), 'duplicate pin')
    actual = [pin(Path(r['path'])) for r in rows]
    require(actual == rows, 'source bytes changed')
    return actual

def runtime():
    require(sha(RUNTIME) == 'fa3e9838c66480557ed6d928a2d159ebc074d05f67f15ef08ab20388d9c49e0b', 'runtime manifest changed')
    m = read(RUNTIME)
    for root, expected in m['treeMembers'].items():
        require(sorted({str(p.resolve()) for p in Path(root).rglob('*') if p.is_file()}) == expected,
                'runtime tree membership changed')
    refs = [pin(RUNTIME), pin(RUNTIME.parent / 'runtime.tar.gz'),
            pin(Path(m['reusedPythonManifest'])), pin(Path(m['reusedPythonArchive']))]
    require([r['sha256'] for r in refs[1:]] == [m['archiveSha256'],
            m['reusedPythonManifestSha256'], m['reusedPythonArchiveSha256']], 'runtime archive references changed')
    require(len(m['files']) == 4758 and len(m['pythonFiles']) == 3329, 'runtime inventory counts')
    expected = {}
    for row in m['files'] + m['pythonFiles']:
        require(row['path'] not in expected or expected[row['path']] == row['sha256'], 'inconsistent shared pin')
        expected[row['path']] = row['sha256']
    files = []
    for name, wanted in sorted(expected.items()):
        actual = pin(Path(name)); require(actual['sha256'] == wanted, 'runtime file changed: ' + name)
        files.append(actual)
    return {'references': refs, 'files': files, 'runtimeFiles': 4758, 'pythonFiles': 3329}

def prepare():
    require(set(p.name for p in OUT.iterdir()) ==
            {'observer.cjs', 'launch.cjs', 'controls.cjs', 'record.py', 'review-implementation.md', 'transport'},
            'fresh directory with four sources and independent implementation review required')
    require((OUT / 'review-implementation.md').read_text().startswith('PASS\n'), 'root implementation review required')
    blocks = dict(re.findall(r'^<!-- file: ([^ ]+) -->\n```(?:javascript|python)\n(.*?)^```$', PLAN.read_text(), re.S | re.M))
    require(set(blocks) == {'observer.cjs', 'launch.cjs', 'controls.cjs', 'record.py'}, 'exact four plan blocks')
    for name, source in blocks.items():
        require((OUT / name).read_bytes() == source.encode(), 'implementation differs from reviewed plan')
    fixed = {
        DESIGN: '19ff70c32004d824ee1b5e34e722425a354412d3548f321a943b6e32fc9ca67a',
        PLANNING / 'design-adoption.json': 'dfc706f891a44c2260f40e68fb2fd2f8b903f1448d7c603cbfcf561ae624fcb2',
        VIEW / 'view-manifest.json': '5b158d684be97052eb362f217c25f1fbaf03583fbd67fd46c5972b0662dcf1a2',
        OLD / 'task2-frozen-source.json': '97fad04118219a75017a738eb7dc4fb013d351161892cfa05e26c26528f55ab2',
        VIEW / 'compile-original/input.json': 'e61e0d68cfe73ca9b2f1e342a59c04637ba870beed781ece2517ec409ac1f026',
        ALIAS / 'original-file-hash-index.json': '7dc430fd098e7fded8ebac492cfbda9d486ffa863802bf830e21ef330e563fde',
        ALIAS / 'direct-compile/input.json': '94ca8f9d328c44aee9e32e212dd05fa8b1c41445600394e9381186fc2c6f2a01',
        ALIAS / 'original-compile/input.json': '8d029348ef1fa478c416bc479e56b13f466a866b899e98741251fca23538657b',
        RH: RH_SHA, A4 / 'recorder.py': A4_SHA,
        PACKAGE / 'dist/src/cli.js': 'ac12595b1cb7253feec93c79417615c6eb20fc3b6a3df35c5e3530b24e90a501',
        PACKAGE / 'dist/src/cliCommands.js': 'b18672d656782aefde254fe1021dc13fdffdcf8f4ad8709e3126fb5fa293362a',
        PACKAGE / 'node_modules/@sweet-monads/either/cjs/index.js': '4b789da6fbd347942c20977ccc140524dc8297179a3cd1d96a5b36327ad30757',
    }
    view = read(VIEW / 'view-manifest.json'); original = read(OLD / 'task2-frozen-source.json')['files']
    require(len(view['files']) == 24 and len(original) == 30, 'frozen input counts')
    for row in view['files']:
        fixed[ROOT / row['view']] = row['viewSha256']
        fixed[ROOT / row['original']] = row['originalSha256']
    for row in original:
        fixed[ROOT / row['path']] = row['sha256']
    index = read(ALIAS / 'original-file-hash-index.json')
    aliases = [r for r in index['originalFiles'] if r['path'].startswith(str(ALIAS.relative_to(ROOT)) + '/src/')]
    require(len(aliases) == 4, 'four frozen alias-control sources')
    for row in aliases:
        fixed[ROOT / row['path']] = row['sha256']
    for name, wanted in fixed.items():
        require(sha(name) == wanted, 'fixed source changed: ' + str(name))
    for kind, folder in [('full', VIEW / 'compile-original'), ('success', ALIAS / 'direct-compile'), ('error', ALIAS / 'original-compile')]:
        require(read(folder / 'input.json')['argv'] == expected_argv(kind), 'historical effective argv binding')
    paths = set(fixed) | {PLAN, RUNTIME, OUT / 'review-implementation.md'}
    paths.update(OUT / name for name in blocks)
    paths.update(PLANNING.glob('*.md'))
    snapshot = runtime()
    write(OUT / 'freeze.json', {'createdAt': now(), 'preparationHead': head(),
          'sources': [pin(p) for p in sorted(paths)], 'runtime': snapshot,
          'historicalDispatchHead': '43f33721da7a3bfb11dd1a91f1f548d5110ab93c',
          'runtimeBootstrapHead': '900bb2051225b4a3d99bf422c3b2e5e386e3e7bc'})
    print(json.dumps({'ok': True, 'scope': 'prepared source only', 'freeze': pin(OUT / 'freeze.json')}))

def environment(mock):
    env = dict(os.environ)
    removed = ['NODE_PATH', 'NODE_COMPILE_CACHE', 'PYTHONPATH', 'JAVA_TOOL_OPTIONS', '_JAVA_OPTIONS',
               'JDK_JAVA_OPTIONS', 'LD_PRELOAD', 'LD_LIBRARY_PATH', 'PYTHONOPTIMIZE']
    for key in removed:
        env.pop(key, None)
    fixed = {'PATH': '/usr/lib/jvm/java-25-openjdk-amd64/bin:' + str(Path(NODE).parent) + ':/usr/bin:/bin',
        'APALACHE_JAR': '/home/charl/.quint/apalache-dist-0.56.1/apalache/lib/apalache.jar',
        'JVM_ARGS': '-Xmx4096m',
        'JVM_GC_ARGS': '-XX:+UseG1GC -XX:G1PeriodicGCInterval=600000 -XX:+G1PeriodicGCInvokesConcurrent',
        'NODE_OPTIONS': '--max-old-space-size=128 --unhandled-rejections=strict' if mock else '--max-old-space-size=4096',
        'NODE_DISABLE_COMPILE_CACHE': '1', 'PYTHONNOUSERSITE': '1', 'PYTHONDONTWRITEBYTECODE': '1'}
    env.update(fixed)
    return env, fixed, removed

def transport(mode):
    folder = OUT / 'transport' / mode
    result = read(folder / 'terminal.json')
    require(result['command'] == command(mode) and result['cwd'] == str(ROOT), 'outer command binding')
    responses = [read(p) for p in sorted(folder.glob('response-*.json'))]
    require(responses == result['toolResponses'] and responses and
            type(responses[-1].get('exit_code')) is int and
            result['wrapperExitCode'] == responses[-1]['exit_code'], 'authentic outer terminal required')
    return [pin(p) for p in sorted(folder.iterdir()) if p.is_file()]

def command(mode):
    return 'PYTHONOPTIMIZE=0 ' + PYTHON + ' -B ' + str(OUT / 'record.py') + ' ' + mode

def verify_stage(mode):
    terminal = read(OUT / mode / 'terminal.json')
    require(terminal['recorderExit'] == 0 and terminal['finalized'], 'previous recorder gate failed')
    check_pins(terminal['ownedFiles'])
    pins = transport(mode)
    require(read(OUT / 'transport' / mode / 'terminal.json')['wrapperExitCode'] == 0, 'previous actual outer gate failed')
    return pins + [pin(OUT / mode / 'terminal.json')]

def adopt(mode):
    review = OUT / ('review-' + mode + '.md')
    require(review.read_text().startswith('PASS\n'), 'separate root review required')
    files = verify_stage(mode) + [pin(review), pin(OUT / 'freeze.json')]
    write(OUT / ('adopt-' + mode + '.json'), {'rootDecision': 'admit wrapper controls only',
          'createdAt': now(), 'head': head(), 'files': files})
    print(json.dumps({'ok': True, 'adopted': mode, 'scope': 'wrapper controls only'}))

def verify_adoption(mode):
    a = read(OUT / ('adopt-' + mode + '.json'))
    check_pins(a['files']); verify_stage(mode)

def a4_terminal():
    folder = A4 / 'fresh97'
    terminal = read(folder / 'terminal.json')
    outer = read(folder / 'outer-tool-receipt.json')
    require(type(terminal['exitCode']) is int and not terminal['artifactFinalizationBlocked'], 'A4 not terminal/clean')
    require(outer['args']['workdir'] == str(ROOT) and outer['responses'] and
            type(outer['responses'][-1].get('exit_code')) is int and
            outer['responses'][-1]['exit_code'] == terminal['exitCode'],
            'A4 actual outer terminal missing')
    for lifecycle in terminal['commandLifecycles']:
        require(not lifecycle['artifactFinalizationBlocked'] and lifecycle['cleanup']['complete'], 'A4 uncertain writer')
        if lifecycle['ownedPgid'] is not None:
            require(not group_exists(lifecycle['ownedPgid']), 'A4 group still present; root must resolve identity')
    return [pin(folder / 'terminal.json'), pin(folder / 'outer-tool-receipt.json')]

def authorize_full():
    verify_adoption('mocks'); verify_adoption('aliases')
    review = OUT / 'review-dispatch.md'
    require(review.read_text().startswith('PASS\n'), 'separate root full-dispatch review required')
    require(not (OUT / 'full').exists(), 'full slot already used')
    files = a4_terminal() + [pin(review), pin(OUT / 'freeze.json'),
            pin(OUT / 'adopt-mocks.json'), pin(OUT / 'adopt-aliases.json')]
    write(OUT / 'root-dispatch.json', {'createdAt': now(), 'actualDispatchHead': head(), 'files': files,
          'argv': expected_argv('full'), 'wallSeconds': 900, 'nodeHeapMiB': 4096, 'jvmHeapMiB': 4096,
          'invocations': 1, 'scope': 'one original-view diagnostic observation only; H1 unresolved'})
    print(json.dumps({'ok': True, 'scope': 'separate root dispatch recorded; no native launch'}))

def trace_info(folder, code):
    path = folder / 'trace.jsonl'
    try:
        require(path.is_file() and not path.is_symlink(), 'missing trace')
        with path.open('rb') as stream:
            raw = stream.read(16385)
        require(0 < len(raw) <= 16384 and raw.endswith(b'\n'), 'trace byte bound/termination')
        def unique(pairs):
            value = {}
            for key, item in pairs:
                require(key not in value, 'duplicate trace key')
                value[key] = item
            return value
        rows = [json.loads(line, object_pairs_hook=unique) for line in raw.splitlines()]
        require(len(rows) <= 32, 'trace count bound')
        next_phase, pending, previous, last_entered, last_completed = 0, None, -1, None, None
        final_error, output_outcome = False, 'Right'
        for number, row in enumerate(rows, 1):
            require(set(row) == {'seq', 'phase', 'event', 'outcome', 'ns'} and
                    type(row['seq']) is int and row['seq'] == number, 'trace schema/sequence')
            require(type(row['ns']) is str and re.fullmatch(r'[0-9]+', row['ns']), 'trace clock encoding')
            ns = int(row['ns']); require(ns >= previous, 'trace clock order'); previous = ns
            if row['event'] == 'enter':
                require(not final_error and pending is None and next_phase < 6 and row['phase'] == PHASES[next_phase], 'phase entry order')
                require(row['outcome'] == (output_outcome if next_phase == 5 else 'stage'), 'entry classification')
                pending = row['phase']; last_entered = row['phase']
            else:
                require(pending == row['phase'] and pending != 'outputResult', 'completion without valid entry')
                require((row['event'] == 'resolve' and row['outcome'] in ('Left', 'Right')) or
                        (row['event'], row['outcome']) in [('throw', 'thrown'), ('reject', 'rejected')], 'completion classification')
                last_completed = {'phase': pending, 'event': row['event'], 'outcome': row['outcome']}
                pending = None
                if row['outcome'] == 'Left':
                    next_phase, output_outcome = 5, 'Left'
                elif row['outcome'] == 'Right':
                    next_phase += 1
                else:
                    final_error = True
        require(code != 74, 'observation failure exit 74')
        return {'valid': True, 'rows': rows, 'lastEntered': last_entered, 'lastCompleted': last_completed,
                'incompleteObservedInterval': pending, 'locationMeaning': 'includes observer I/O and scheduling margins'}
    except Exception as exc:
        return {'valid': False, 'lastEntered': None, 'lastCompleted': None,
                'incompleteObservedInterval': None, 'reason': type(exc).__name__ + ': ' + str(exc)}

def spawn(folder, argv, wall, mock, sources, lifecycle):
    folder.mkdir()
    source_bytes = archive(folder / 'source.tar.gz', [Path(r['path']) for r in sources])
    env, fixed, removed = environment(mock)
    measured = ['/usr/bin/time', '-v', '-o', str(folder / 'resources.txt'), *argv]
    write(folder / 'start.json', {'createdAt': now(), 'actualDispatchHead': head(), 'argv': argv,
          'measuredArgv': measured, 'cwd': str(ROOT), 'wallSeconds': wall, 'fixedEnvironment': fixed,
          'removedEnvironmentKeys': removed, 'parentOrigArgv': sys.orig_argv,
          'sourceArchive': pin(folder / 'source.tar.gz'), 'sourceBytes': source_bytes})
    child, problem, timed_out = None, None, False
    cleanup = {'forced': False, 'signals': [], 'errors': [], 'complete': False}
    phase, received = 'launch', []
    lifecycle.update({'finalizationBlocked': True, 'childExit': None, 'ownedPgid': None})
    def interrupted(signum, frame):
        received.append(signal.Signals(signum).name)
        if phase == 'wait':
            raise InterruptedError('recorder interrupted')
    previous = {sig: signal.signal(sig, interrupted) for sig in (signal.SIGINT, signal.SIGTERM)}
    started = time.monotonic()
    try:
        with (folder / 'stdout.bin').open('xb') as stdout, (folder / 'stderr.bin').open('xb') as stderr:
            try:
                child = subprocess.Popen(measured, cwd=ROOT, env=env, stdout=stdout, stderr=stderr, start_new_session=True)
                lifecycle['ownedPgid'] = child.pid
                phase = 'wait'
                if received:
                    raise InterruptedError('recorder interrupted during spawn')
                write(folder / 'process.json', {'pid': child.pid, 'ownedPgid': child.pid, 'parentPid': os.getpid(),
                      'startNewSession': True, 'recordedAt': now(), 'meaning': 'spawn identity; not present liveness'})
                remaining = max(0, wall - (time.monotonic() - started))
                child.wait(timeout=remaining)
            except subprocess.TimeoutExpired:
                timed_out = True
            except BaseException as exc:
                problem = type(exc).__name__ + ': ' + str(exc)
            finally:
                phase = 'cleanup'
                if child is None:
                    cleanup['complete'] = True
                else:
                    observation_error = None
                    try:
                        present = group_exists(child.pid)
                    except BaseException as exc:
                        present = True
                        observation_error = type(exc).__name__ + ': ' + str(exc)
                    try:
                        if timed_out or problem or received or child.returncode is None or present:
                            cleanup = cleanup_group(child, 5, 5)
                        else:
                            cleanup['complete'] = True
                        if observation_error is not None:
                            cleanup['errors'].append(observation_error)
                            cleanup['complete'] = False
                    except BaseException as exc:
                        cleanup['errors'].append(type(exc).__name__ + ': ' + str(exc))
                        cleanup['complete'] = False
    except BaseException as exc:
        problem = problem or type(exc).__name__ + ': ' + str(exc)
    finally:
        for sig, handler in previous.items():
            signal.signal(sig, handler)
    code = child.returncode if child is not None else None
    complete = cleanup['complete'] and not cleanup['errors']
    # Publish before any terminal serialization, stream hash, or source check.
    lifecycle.update({'finalizationBlocked': not complete, 'childExit': code,
          'ownedPgid': child.pid if child else None, 'cleanup': cleanup, 'timedOut': timed_out,
          'monitorError': problem, 'receivedSignals': received})
    write(folder / 'process-terminal.json', dict(lifecycle, completedAt=now(), elapsedSeconds=time.monotonic() - started))
    require(complete, 'owned process group uncertain: root takeover before hashing')
    check_pins(sources)
    resource, resource_error = None, None
    try:
        resource = group['parse_resource'](folder / 'resources.txt', argv)
    except Exception as exc:
        resource_error = type(exc).__name__ + ': ' + str(exc)
    result = dict(lifecycle, resource=resource, resourceError=resource_error,
                  stdout=pin(folder / 'stdout.bin'), stderr=pin(folder / 'stderr.bin'))
    write(folder / 'result.json', result)
    return result

def ordinary(result, code):
    require(result['childExit'] == code and not result['timedOut'] and not result['cleanup']['forced']
            and not result['monitorError'] and not result['receivedSignals'] and result['cleanup']['complete'], 'native terminal mismatch')
    require(result['resource'] is not None and result['resource']['exit_status'] == code, 'GNU-time exit binding')

def execute(mode):
    require(mode in ('mocks', 'aliases', 'full'), 'unknown execution mode')
    freeze = read(OUT / 'freeze.json')
    check_pins(freeze['sources'])
    if mode in ('aliases', 'full'):
        verify_adoption('mocks')
    if mode == 'full':
        verify_adoption('aliases'); a4_terminal()
        dispatch = read(OUT / 'root-dispatch.json'); check_pins(dispatch['files'])
        require(dispatch['actualDispatchHead'] == head() and dispatch['argv'] == expected_argv('full')
                and dispatch['wallSeconds'] == 900 and dispatch['invocations'] == 1, 'separate root dispatch mismatch')
    stage = OUT / mode
    stage.mkdir()
    life, error, code, before, after, checks = [], None, 1, None, None, None
    sources = freeze['sources'] + [pin(OUT / 'freeze.json')]
    if mode in ('aliases', 'full'):
        sources += [pin(OUT / 'adopt-mocks.json'), pin(OUT / 'review-mocks.md')]
    if mode == 'full':
        sources += [pin(OUT / 'adopt-aliases.json'), pin(OUT / 'review-aliases.md'),
                    pin(OUT / 'root-dispatch.json'), pin(OUT / 'review-dispatch.md')]
    try:
        before = runtime(); require(before == freeze['runtime'], 'runtime differs from preparation')
        write(stage / 'runtime-before.json', before)
        write(stage / 'start.json', {'createdAt': now(), 'mode': mode, 'head': head(), 'sources': sources,
              'parentOrigArgv': sys.orig_argv, 'scope': 'diagnostic observation only; H1 unresolved'})
        if mode == 'mocks':
            records = []
            for case in ['identity', 'open', 'write', 'partial', 'records', 'bytes', 'classify']:
                folder = stage / case; lifecycle = {}; life.append(lifecycle)
                argv = [NODE, str(OUT / 'controls.cjs'), case, str(folder)]
                result = spawn(folder, argv, 15, True, sources, lifecycle)
                ordinary(result, 0 if case == 'identity' else 74)
                stdout, stderr = (folder / 'stdout.bin').read_bytes(), (folder / 'stderr.bin').read_bytes()
                if case == 'identity':
                    report = json.loads(stdout)
                    require(report['ok'] and [r['mode'] for r in report['controls']] ==
                            ['sync-right', 'sync-left', 'resolve', 'reject', 'throw', 'right-chain', 'left-chain']
                            and stderr == b'', 'identity matrix failed')
                else:
                    require(stdout == b'' and stderr == b'phase-observer: diagnostic failure\n', 'fault did not fail closed')
                    called = folder / 'original-called.txt'
                    require(called.read_bytes() == b'1\n' if case == 'classify' else not called.exists(), 'fault original-call predicate')
                    if case == 'records':
                        require(len((folder / 'trace.jsonl').read_bytes().splitlines()) == 32, 'record bound fault')
                    if case == 'bytes':
                        require((folder / 'trace.jsonl').read_bytes() == b'', 'byte bound fault')
                records.append({'case': case, 'result': pin(folder / 'result.json'), 'ok': True})
            checks = {'ok': True, 'scope': 'mock observer controls only', 'cases': records}
        else:
            pairs = [('success', False), ('success', True), ('error', False), ('error', True)] if mode == 'aliases' else [('full', True)]
            results = {}
            for kind, instrumented in pairs:
                name = kind + ('-observed' if instrumented else '-direct')
                folder = stage / name; lifecycle = {}; life.append(lifecycle)
                effective = expected_argv(kind)
                if instrumented:
                    # Invocation is immutable, owned by the parent stage, and archived
                    # with this child. The child directory remains exclusive for spawn.
                    invocation = stage / (name + '-invocation.json')
                    write(invocation, {'effectiveArgv': effective, 'trace': str(folder / 'trace.jsonl'),
                          'argvReceipt': str(folder / 'argv.json')})
                    argv = [NODE, str(OUT / 'launch.cjs'), str(invocation)]
                    child_sources = sources + [pin(invocation)]
                else:
                    argv, child_sources = effective, sources
                result = spawn(folder, argv, 900 if mode == 'full' else 120, False, child_sources, lifecycle)
                if instrumented:
                    trace = trace_info(folder, result['childExit'])
                    actual = read(folder / 'argv.json') if (folder / 'argv.json').is_file() else None
                    argv_ok = (actual is not None and actual['actualLaunchArgv'] == argv and
                               actual['effectiveArgv'] == effective and actual['execPath'] == NODE and
                               actual['execArgv'] == [] and actual['cwd'] == str(ROOT))
                    if not argv_ok or type(result['childExit']) is not int or result['monitorError'] or result['receivedSignals']:
                        trace = {'valid': False, 'lastEntered': None, 'lastCompleted': None,
                                 'incompleteObservedInterval': None, 'reason': 'argv or recorder interruption'}
                    write(folder / 'trace-validation.json', {'argvMatched': argv_ok, **trace})
                    if mode == 'aliases':
                        require(trace['valid'] and argv_ok, 'baseline trace/argv rejected')
                        wanted = PHASES if kind == 'success' else [p for p in PHASES if p != 'outputCompilationTarget']
                        require([r['phase'] for r in trace['rows'] if r['event'] == 'enter'] == wanted, 'baseline phase order')
                        require(trace['rows'][-1]['phase'] == 'outputResult' and
                                trace['rows'][-1]['outcome'] == ('Right' if kind == 'success' else 'Left'), 'baseline final classification')
                if mode == 'aliases':
                    ordinary(result, 0 if kind == 'success' else 1)
                    if not instrumented:
                        if kind == 'success':
                            parsed = json.loads((folder / 'stdout.bin').read_bytes())
                            require(len(parsed['modules']) == 1 and parsed['main'] == 'alias_visibility_direct', 'successful JSON shape')
                            require({'q::init', 'q::step', 'q::inv'} <= {d['name'] for d in parsed['modules'][0]['declarations']}, 'selected declarations')
                            require((folder / 'stderr.bin').read_bytes() == b'', 'successful stderr')
                        else:
                            require((folder / 'stdout.bin').read_bytes() == b'' and b'name resolution failed' in (folder / 'stderr.bin').read_bytes(), 'genuine missing-alias baseline')
                    else:
                        direct = results[kind + '-direct']['folder']
                        for stream_name in ('stdout.bin', 'stderr.bin'):
                            require((direct / stream_name).read_bytes() == (folder / stream_name).read_bytes(), 'baseline raw output mismatch')
                results[name] = {'folder': folder, 'result': result}
            if mode == 'aliases':
                checks = {'ok': True, 'scope': 'four wrapper baseline controls only', 'runs': list(results),
                          'rawPairsEqual': True, 'successfulJSONByteIdentical': True}
            else:
                only = results['full-observed']; result = only['result']
                require(type(result['childExit']) is int and not result['monitorError'] and
                        not result['receivedSignals'], 'full launch or recorder monitor failed; preserve without admission')
                checks = {'ok': True, 'scope': 'preserved diagnostic observation only',
                          'compilerAcceptance': False, 'H1': 'unresolved', 'childExit': result['childExit'],
                          'timedOut': result['timedOut'], 'observationFailureExit74': result['childExit'] == 74,
                          'trace': read(only['folder'] / 'trace-validation.json')}
        write(stage / 'checks.json', checks)
        code = 0
    except BaseException as exc:
        error = type(exc).__name__ + ': ' + str(exc)
    finally:
        blocked = any(r.get('finalizationBlocked', True) for r in life)
        if not blocked:
            try:
                check_pins(sources)
                after = runtime(); require(before == after, 'runtime changed during stage')
                write(stage / 'runtime-after.json', after)
            except BaseException as exc:
                code = 2; error = (error or '') + '; after-check: ' + str(exc)
        else:
            code = 2; error = (error or '') + '; root takeover required; no artifact hashes finalized'
        owned = []
        if not blocked:
            try:
                for p in sorted(stage.rglob('*')):
                    require(not p.is_symlink(), 'unexpected output symlink')
                    if p.is_file():
                        owned.append(pin(p))
            except BaseException as exc:
                code = 2; error = (error or '') + '; artifact-finalization: ' + str(exc)
                owned = []
        terminal = {'recorderExit': code, 'completedAt': now(), 'error': error, 'lifecycles': life,
                    'finalized': not blocked and bool(owned), 'ownedFiles': owned,
                    'scope': 'preservation/control gate only; never H1 or compiler acceptance'}
        write(stage / 'terminal.json', terminal)
        print(json.dumps({'mode': mode, 'recorderExit': code, 'finalized': terminal['finalized'], 'error': error}), flush=True)
    return code

def intake():
    verify_adoption('mocks'); verify_adoption('aliases'); verify_stage('full')
    review = OUT / 'review-final.md'
    require(review.read_text().startswith('PASS\n'), 'independent final preservation review required')
    for mode in ('mocks', 'aliases', 'full'):
        for item in read(OUT / mode / 'terminal.json')['lifecycles']:
            require(not item['finalizationBlocked'] and item['cleanup']['complete'], 'unresolved diagnostic group')
            if item['ownedPgid'] is not None:
                require(not group_exists(item['ownedPgid']), 'group identity requires root resolution')
    require(not (OUT / 'intake.json').exists(), 'intake already exists')
    files, directories = [], []
    for path in sorted(OUT.rglob('*')):
        # Intake's own transport stays incremental until this process exits.
        if OUT / 'transport/intake' in path.parents or path == OUT / 'transport/intake':
            continue
        require(not path.is_symlink(), 'unexpected diagnostic symlink')
        if path.is_file():
            files.append(pin(path))
        elif path.is_dir():
            directories.append(str(path))
        else:
            raise RuntimeError('unexpected diagnostic artifact')
    write(OUT / 'intake.json', {'createdAt': now(), 'scope': 'preserved diagnostic only; H1 unresolved',
          'fullObservation': read(OUT / 'full/checks.json'), 'files': files, 'directories': directories,
          'selfExcluded': True, 'incrementalIntakeTransportExcluded': True,
          'historicalCompilerPhase': 'unknown', 'compilerAcceptance': False})
    print(json.dumps({'ok': True, 'scope': 'preservation intake only', 'intake': pin(OUT / 'intake.json')}))

def main():
    require(len(sys.argv) == 2 and sys.argv[1] in MODES, 'one finite mode required')
    require(sys.orig_argv == [PYTHON, '-B', str(OUT / 'record.py'), sys.argv[1]], 'exact recorder argv required')
    mode = sys.argv[1]
    if mode == 'prepare':
        prepare()
    elif mode.startswith('adopt-'):
        adopt(mode.removeprefix('adopt-'))
    elif mode == 'authorize-full':
        authorize_full()
    elif mode == 'intake':
        intake()
    else:
        return execute(mode)
    return 0

if __name__ == '__main__':
    raise SystemExit(main())
```

- [ ] Review CP001 before any execution. Check the complete JavaScript call graph against installed `cli.js:59–60,119–124`, `cliCommands.js:47–56,533–562`, and the pinned Either implementation's `asyncChain`. Check every Python post-Popen path against the admitted cleanup source. No helper globals are reassigned. `capture()` is deliberately not reused because its acceptance predicate requires an unrelated inner receipt and it finalizes hashes even when cleanup is incomplete; only its independently admitted `cleanup_group`, `group_exists`, and `parse_resource` functions are reused. The A4 recorder contributes only its pure pin, exclusive JSON read/write, and source-archive functions.
- [ ] Treat source-review defects as plan defects. Correct this document and obtain root re-review before materialization. Do not patch the installed runtime or create an alternative diagnostic wrapper outside these blocks.

## Task CP002: Materialize, pin, and run the seven focused mock controls

**Files:** The four CP001 files, `review-implementation.md`, `freeze.json`, `mocks/`, and `transport/` under `.superpowers/sdd/a5-compiler-phase-diagnostic-20260906/`.

**Interfaces:** `prepare` requires exactly the four source files, the independent implementation review, and the transport directory. It checks exact plan-block bytes and all fixed input references. `mocks` consumes `freeze.json`, emits seven child directories plus stage start/checks/terminal and before/after runtime snapshots. It never invokes Quint or a model.

- [ ] After root plan approval, run this source-only materialization command from the worktree. Expected: four files created; no Node, Quint, test, or compiler invocation. A preexisting destination fails without reuse.

```sh
PYTHONOPTIMIZE=0 /home/charl/Moriarty/.venv/bin/python -B - <<'PY'
from pathlib import Path
import re
root = Path('/home/charl/Moriarty/.worktrees/s01-audit-start')
plan = root / 'docs/superpowers/plans/2026-09-06-candidate-a-compiler-phase-diagnostic.md'
out = root / '.superpowers/sdd/a5-compiler-phase-diagnostic-20260906'
blocks = re.findall(r'^<!-- file: ([^ ]+) -->\n```(?:javascript|python)\n(.*?)^```$', plan.read_text(), re.S | re.M)
if len(blocks) != 4 or {name for name, source in blocks} != {'observer.cjs', 'launch.cjs', 'controls.cjs', 'record.py'}:
    raise SystemExit('Plan source block inventory mismatch')
out.mkdir(exist_ok=False)
for name, source in blocks:
    with (out / name).open('xb') as stream:
        stream.write(source.encode())
print('Created four source files only')
PY
```

- [ ] Root independently reviews the materialized bytes and code before writing the implementation PASS record. This exact command is a root review action, not an implementer self-approval. If review fails, retain the finding and stop before `prepare`.

```sh
/home/charl/Moriarty/.venv/bin/python -B -c 'from pathlib import Path; Path("/home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a5-compiler-phase-diagnostic-20260906/review-implementation.md").open("x").write("PASS\nRoot independently reviewed the exact four materialized source blocks against the adopted design and reviewed plan. Callback and promise identity, diagnostic failure handling, exclusive paths, resource limits, all-path group cleanup, source/runtime pins and receipt gates are acceptable for the specified controls only. No native result or full dispatch is admitted.\n")'
```

- [ ] Use the following **exact transport function body** in `functions.exec` for every recorder invocation in this plan. Append the exact `await runDiagnosticTransport(...)` call specified at each step to this body in that same tool call. This is one defined tool interface, not a shell script installed into the worktree. The function preserves each original `exec_command`/`write_stdin` response before requesting the next response. It does not parse or replace the recorder's outputs and never synthesizes an exit when transport ends without a terminal.

```javascript
async function runDiagnosticTransport(mode) {
  const root = '/home/charl/Moriarty/.worktrees/s01-audit-start';
  const out = root + '/.superpowers/sdd/a5-compiler-phase-diagnostic-20260906';
  if (!['prepare', 'mocks', 'adopt-mocks', 'aliases', 'adopt-aliases', 'authorize-full', 'full', 'intake'].includes(mode)) throw Error('mode');
  const command = 'PYTHONOPTIMIZE=0 /home/charl/Moriarty/.venv/bin/python -B ' + out + '/record.py ' + mode;
  const quote = value => "'" + value.replaceAll("'", "'\\''") + "'";
  const writeCode = 'import sys; from pathlib import Path; p=Path(sys.argv[1]); p.parent.mkdir(parents=True,exist_ok=True); f=p.open("x",encoding="utf-8"); f.write(sys.argv[2]); f.close()';
  async function save(name, value) {
    const result = await tools.exec_command({
      cmd: '/home/charl/Moriarty/.venv/bin/python -B -c ' + quote(writeCode) + ' ' +
        quote(out + '/transport/' + mode + '/' + name) + ' ' + quote(JSON.stringify(value, null, 2) + '\n'),
      workdir: root, max_output_tokens: 300, yield_time_ms: 1000});
    if (result.exit_code !== 0) throw Error('Transport preservation failed; root takeover required');
  }
  const args = {cmd: command, workdir: root, max_output_tokens: 2500, yield_time_ms: 1000};
  await save('command.json', {command, cwd: root, args});
  const responses = [];
  let response = await tools.exec_command(args);
  while (true) {
    await save('response-' + String(responses.length).padStart(3, '0') + '.json', response);
    responses.push(response);
    if (typeof response.exit_code === 'number') break;
    if (typeof response.session_id !== 'number') throw Error('No terminal or live session; preserve missing terminal and stop');
    notify({mode, status: 'running', session_id: response.session_id});
    response = await tools.write_stdin({session_id: response.session_id, chars: '', yield_time_ms: 10000, max_output_tokens: 2500});
  }
  await save('terminal.json', {command, cwd: root, toolResponses: responses, wrapperExitCode: response.exit_code});
  text({mode, actualOuterExit: response.exit_code, originalResponses: responses.length});
}
```

If a `functions.exec` cell yields, resume that same cell with `functions.wait`; do not launch a second recorder. Tool waits are at most 10 seconds per poll. A failed transport save stops the transport loop and requires root takeover of the known session/group; do not use this failure as permission to launch another command. Preserve any actual tool failure in a fresh root note and account for missing records. Stage `terminal.json` may be absent if startup or serialization fails; a fabricated replacement is forbidden.

- [ ] Run preparation through the transport body with this exact final line:

```javascript
await runDiagnosticTransport('prepare');
```

The exact child command is:

```sh
PYTHONOPTIMIZE=0 /home/charl/Moriarty/.venv/bin/python -B /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a5-compiler-phase-diagnostic-20260906/record.py prepare
```

Expected: actual outer 0 and `{"ok":true,"scope":"prepared source only",...}`. `freeze.json` binds current preparation HEAD, source bytes, exact runtime inventories and external archive digests. Preparation does not authorize the full diagnostic.

- [ ] Root authorizes the small mock gate after CP001 review. Run through the same defined transport body:

```javascript
await runDiagnosticTransport('mocks');
```

Exact child command:

```sh
PYTHONOPTIMIZE=0 /home/charl/Moriarty/.venv/bin/python -B /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a5-compiler-phase-diagnostic-20260906/record.py mocks
```

The recorder invokes `[NODE, str(OUT / 'controls.cjs'), case, str(OUT / 'mocks' / case)]` for the seven exact cases below. `NODE` and `OUT` are the fixed absolute constants defined in CP001. The measured command prepends `['/usr/bin/time', '-v', '-o', str(OUT / 'mocks' / case / 'resources.txt')]`. There are seven processes; none loads the real CLI.

| Child directory under `mocks/` | Control argument | Predicate |
| --- | --- | --- |
| `identity` | `identity` | Exit 0, empty stderr, seven identity/chain cases pass; strict rejection mode survives a further event-loop turn. |
| `open` | `open` | Exit 74; trace path is the intentionally precreated directory; original function uncalled. |
| `write` | `write` | Exit 74 on throwing sink; original uncalled. |
| `partial` | `partial` | Exit 74 on an actual partial record write; preserve malformed original bytes; original uncalled. |
| `records` | `records` | Exit 74 before record 33; exactly 32 original records remain. |
| `bytes` | `bytes` | Exit 74 before writing a record exceeding 16 KiB; trace remains empty. |
| `classify` | `classify` | Original runs exactly once; unsupported result fails classification with exit 74; its one-call file remains original. |

All six faults must have empty stdout and exactly `phase-observer: diagnostic failure\n` on stderr. Their expected 74 is a successful **fault control predicate**, not a successful compiler result. All seven require actual resource/child exits, clean unforced groups, no timeout or recorder interruption, and stable source/runtime endpoints. Any failed predicate stops this gate immediately; no remaining control or native command runs.

- [ ] Independent root intake checks the raw identity report, strict-mode start receipts, six original faults, all process/cleanup/resource records, source archives and before/after pins, stage `checks.json` and `terminal.json`, and the original outer terminal. Root then writes and records adoption with these exact actions; source predicates alone cannot self-adopt.

```sh
/home/charl/Moriarty/.venv/bin/python -B -c 'from pathlib import Path; Path("/home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a5-compiler-phase-diagnostic-20260906/review-mocks.md").open("x").write("PASS\nRoot independently admits the seven exact mock controls from their original identity/fault outputs, authentic child and outer terminals, clean groups, and stable archived source/runtime bindings. This admits observer controls only and authorizes no full diagnostic.\n")'
```

```javascript
await runDiagnosticTransport('adopt-mocks');
```

Expected: actual outer 0 and new `adopt-mocks.json`, which binds the review, stage's finalized originals, original outer transport and preparation freeze. Root review must not write PASS if any record is absent, malformed, unresolved, or outside the fixed control inventory.

## Task CP003: Four fresh tiny direct/instrumented baseline compilations

**Files:** `aliases/`, `review-aliases.md`, `adopt-aliases.json`, and corresponding transport directories inside the dedicated diagnostic directory. The four frozen files under `.superpowers/sdd/a5-factoring-receipts/alias-visibility-control/src/` are read only.

**Interfaces:** `aliases` requires verified mock adoption and source/runtime stability. It produces exactly four child invocations, invocation/argv originals for instrumented children, two raw comparison predicates, trace predicates and final receipts.

Repository observation: previous controls show one tiny successful alias-visible input and one genuine compile-only missing-alias input. Inference: the wrapper is a changed launch path, so fresh direct and instrumented compilations of the **same** frozen inputs test that change more directly than replaying earlier standalone typecheck/prefix/sample gates. These four runs are explicitly justified wrapper controls and provide no new model or H1 claim.

- [ ] After root admits CP002 and separately authorizes this small native gate, run:

```javascript
await runDiagnosticTransport('aliases');
```

Exact child command:

```sh
PYTHONOPTIMIZE=0 /home/charl/Moriarty/.venv/bin/python -B /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a5-compiler-phase-diagnostic-20260906/record.py aliases
```

Execution order is exactly `success-direct`, `success-observed`, `error-direct`, `error-observed`. Each has a fresh child directory and 120-second/4096-MiB settings. Direct effective argv is the exact archived original argv for `driver_direct.qnt` or `driver_original.qnt`. Instrumented actual argv is `[NODE, str(OUT / 'launch.cjs'), str(OUT / 'aliases' / (name + '-invocation.json'))]` with `name` exactly `success-observed` or `error-observed`; that pinned invocation restores the corresponding exact effective CLI argv. No direct/instrumented pair shares an output path. Each direct child's raw/content predicates run immediately after its terminal check; each observed child's pair equality runs before the loop can launch another child. A failure exits into the existing terminal/finalization path and stops the sequence; no repeat is supplied.

- [ ] Independently inspect these required predicates:

| Pair | Actual exit | Raw outputs | Exact entered phases |
| --- | --- | --- | --- |
| Success direct/observed | 0 / 0 | Byte-identical stdout containing successful JSON; both stderr files empty. JSON has one main module `alias_visibility_direct` and selected `q::init`, `q::step`, `q::inv` declarations. | Observed: `load`, `parse`, `typecheck`, `compile`, `outputCompilationTarget`, `outputResult`; Right settlements and final Right entry. |
| Missing-alias direct/observed | 1 / 1 | Byte-identical empty stdout and byte-identical genuine name-resolution stderr. | Observed: `load`, `parse`, `typecheck`, `compile`, `outputResult`; compile resolves Left, output target is skipped, final entry is Left. |

Both observed runs require exact real/effective argv, regular bounded well-formed trace files, strict sequence/clock/phase/classification checks, authentic GNU-time and process exits, clean unforced groups, and stable full source/runtime bindings. Input source bytes are identical within each pair and remain bound to their admitted original alias-control index. These controls cannot establish arbitrary observer equivalence; they cover the actual pinned success/error CLI routes plus the mock identity obligations.

- [ ] On a complete independent PASS, root writes the review and adopts:

```sh
/home/charl/Moriarty/.venv/bin/python -B -c 'from pathlib import Path; Path("/home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a5-compiler-phase-diagnostic-20260906/review-aliases.md").open("x").write("PASS\nRoot independently admits the four fresh tiny direct/instrumented comparisons: exact frozen inputs, actual 0/0 and 1/1 exits, raw stdout/stderr equality, successful JSON byte identity, correct Right/Left phase orders and argv, clean groups, stable source/runtime pins and original outer terminal. These are wrapper controls only. The full diagnostic still requires A4 terminal and separate root dispatch.\n")'
```

```javascript
await runDiagnosticTransport('adopt-aliases');
```

Expected: actual outer 0 and exclusively created `adopt-aliases.json`. Do not write a PASS review or adoption when only a subset ran.

## Task CP004: Separately authorize and dispatch the full original once

**Files:** `review-dispatch.md`, `root-dispatch.json`, `full/`, and their separate transport records under the diagnostic directory. A4 fresh97 terminal and its original outer receipt are read only.

**Interfaces:** `authorize-full` checks both admitted control gates, the completed A4 process groups and actual terminal, and binds the current HEAD, reviews, inputs and limits. `full` rechecks those bindings and requires an unused full directory; it emits an observation-preservation gate, never compiler acceptance.

- [ ] Wait for A4 fresh97's actual terminal **and** `.superpowers/sdd/a4-task6-resumption-20260906/fresh97/outer-tool-receipt.json`. Its existing interface is `args` plus `responses`; require actual last-response exit agreement with its recorder terminal. A4 success is not inferred from liveness or partial child success. The diagnostic's non-overlap requirement is terminal and clean owned groups, not an invented A4 success. If any group ID still exists or identity is uncertain, stop for root resolution; do not signal an unrelated or reused group.
- [ ] Root reviews the admitted wrapper controls, current frozen inputs, A4 terminal/non-overlap, and exact full command. Root writes this separate decision only when those checks are complete:

```sh
/home/charl/Moriarty/.venv/bin/python -B -c 'from pathlib import Path; Path("/home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a5-compiler-phase-diagnostic-20260906/review-dispatch.md").open("x").write("PASS\nRoot separately authorizes one full original-view compiler phase diagnostic after independent admission of the exact mock and tiny baseline controls and inspection of the actual terminal A4 fresh97 process/outer receipts with no unresolved owned group. Dispatch retains the exact original effective CLI argv, 900-second wall and 4096-MiB Node/JVM settings. This authorizes one diagnostic observation only, no retry, factored compile, solver or H1 admission.\n")'
```

```javascript
await runDiagnosticTransport('authorize-full');
```

Expected: actual outer 0 and `root-dispatch.json` created without any native launch. Its SHA and actual current HEAD become required full-dispatch bindings. Any post-authorization source/HEAD change invalidates dispatch; do not edit the record to bypass the gate.

- [ ] Root separately launches the one full slot:

```javascript
await runDiagnosticTransport('full');
```

Exact recorder command:

```sh
PYTHONOPTIMIZE=0 /home/charl/Moriarty/.venv/bin/python -B /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a5-compiler-phase-diagnostic-20260906/record.py full
```

Exact instrumented child argv, below the GNU-time wrapper:

```text
/home/charl/.foreman/tools/fnm/node-versions/v24.18.1/installation/bin/node
/home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a5-compiler-phase-diagnostic-20260906/launch.cjs
/home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a5-compiler-phase-diagnostic-20260906/full/full-observed-invocation.json
```

The required effective CLI argv is exactly:

```text
/home/charl/.foreman/tools/fnm/node-versions/v24.18.1/installation/bin/node
/home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/cli.js
compile
/home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a5-factoring-receipts/pilot-compilation-view/view/specs/quint/s02/factored_verification/candidate_a_funding_pilot.qnt
--main=candidate_a_funding_pilot
--target=json
--invariant=pilotSafety
--verbosity=0
```

`init` and `step` retain original CLI defaults; no additional compile flag, source-map output, flatten switch, target change or solver is introduced. The exact fixed environment is recorded in `start.json`. Node's old-space and JVM heap settings do not assert a bound on total RSS. Runtime/source hashing and post-child checks are outside the native wall interval; the process-terminal elapsed field includes cleanup. GNU-time data, if complete, retain their own measurement scope.

## Task CP005: Admit only the actual terminal observation and preserve originals

**Files:** `review-final.md`, `intake.json`, and separate `transport/intake/` records inside the diagnostic directory. No old timeout package or frozen input is changed.

**Interfaces:** `full/terminal.json` reports recorder preservation status separately from the GNU-time-wrapped child's actual exit. `full/checks.json` always carries `compilerAcceptance:false` and `H1:unresolved`. Root's `intake` mode admits only independently reviewed preservation, inventories originals, and excludes its own growing outer transport until that actual transport finishes.

- [ ] Inspect raw full `stdout.bin`, `stderr.bin`, trace, invocation and argv receipts, resource file, source archive, start/process/process-terminal/result/trace-validation records, full before/after runtime snapshots, stage terminal, and every original outer response. Verify source archive membership/hashes, exact restored argv, current dispatch versus historical bases, and unchanged 24/30 sources. Preserve even empty, malformed, failed, or partial originals.
- [ ] Interpret terminals with this exact distinction:

| Result | Required handling |
| --- | --- |
| Child completes normally | Preserve actual GNU-time-wrapped child code, resource report, outputs and trace. Valid JSON stays diagnostic output; no paired-input/H1 promotion. |
| 900-second timeout with complete cleanup | Preserve authentic signal/code and timeout flag, even if resources are empty. A valid trace permits only the last entered/completed observed public intervals of this **new** invocation. Recorder outer 0 means preservation succeeded, not compilation. |
| Child 74 | Preserve diagnostic failure and constant error. Phase location is unavailable, even if earlier trace records look valid. It is not a compiler outcome. |
| Missing/malformed trace, argv mismatch, recorder interruption | Retain originals; phase location is unavailable. An authentic signal that preempts logging is not rewritten as 74. |
| Incomplete group, source/runtime drift, startup/handoff/finalization failure | No admission or final intake index. Preserve available process and transport records; cleanly closed files may retain their failed-stage hashes. An uncertain writer blocks even those hashes. Root explicitly accounts for missing files and resolves ownership before any later action. No retry is included. |

An entry without completion identifies only an incomplete **observed interval**, which includes logging I/O and promise-reaction scheduling. It does not prove that the original function was executing at interruption, bound observer I/O latency, identify an internal hot spot, or locate the historical timeout. `outputResult` may have an entry and no completion because the original function exits the process; the authentic native exit remains separate. A completed `compile` marker does not prove serialization/output completed.

- [ ] If preservation has a complete independent PASS, root writes its scoped final review and runs intake. An empty/malformed trace may be preserved with unavailable phase location; a live/uncertain group or failed source/runtime gate cannot receive this PASS.

```sh
/home/charl/Moriarty/.venv/bin/python -B -c 'from pathlib import Path; Path("/home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a5-compiler-phase-diagnostic-20260906/review-final.md").open("x").write("PASS\nRoot independently admits preservation of the one terminal diagnostic invocation and all original process, output, trace, resource, source/runtime and outer transport records. The trace-validation record controls whether phase location is available; it makes no claim about the historical timeout or an internal hotspot. Any retained generated JSON remains diagnostic only. Compiler acceptance is false and H1 remains unresolved.\n")'
```

```javascript
await runDiagnosticTransport('intake');
```

Expected: actual outer 0 and `intake.json` with hashes of every prior original file and an explicit directory inventory. The index excludes itself and its currently growing `transport/intake/`; preserve that actual terminal separately without rewriting the index. A later durable archival package must include the index and that post-index transport together. No durable-package rewrite or commit is authorized by this plan.

## Plan self-review and handoff

- [ ] Root separately reviews this complete plan and its exact SHA before CP001 materialization. The author supplies a source-only self-review; no test result is assumed.
- [x] Author checked specification coverage: six exports and actual CLI binding; receiver/arguments/value/promise identity; Left skips; bounded trace and fail-74; no production handlers or exit marker; exact mock/fault controls; four identical-source direct/observed baselines; source/runtime/archive/argv/process/outer gates; A4 terminal and separate full dispatch; once-only limits; terminal preservation and no H1 promotion.
- [x] Author checked the four complete code-block inventory and Python syntax by read-only AST inspection, and inspected JavaScript statically. No Node, Quint, or Python test/control code ran during planning. The extraction regex anchors fence lines so the Python regex literal cannot terminate its own block. Static review corrected invocation/child-directory binding, initial transport ownership, and the A4 outer `args`/`responses` interface. These are source-review observations, not executed-control results.
- [ ] No commit, branch integration, user confirmation loop, or execution-choice prompt is part of this plan-only delegation. Hand the completed document and exact hash to root for independent review. Root owns all later implementation/control/dispatch decisions.
