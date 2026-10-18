# Actual custody oracle-path CLI source/result review

Exact head b988764129e8828e3bc5b4d4b7660df61c3a184f; tree 32a27d88760401c65be4606527274a15410792ea; parent/base 049395532c43b17c54bdbc5a529535b521dc56df; only the three paths below changed. This input is prepared by the source author, not an approval. A fresh nonauthor ordinary reviewer must assess the full actual committed source and actual receipts. No requested model or provider result is invented; no provider is dispatched by this preparation.

Subject SHA256 4dc0747eba262bce761256bcdc4d2b5bc48049949140f298d84e576870c83758; reviewToken `doctorwatson-custody-oracle-cli-path-source-result-ordinary-20261018-a1-whole-review`. Source correctness and actual-result fidelity must receive separate assessments. The permitted result is ordinary CLI argument transport only. Do not turn this into a Q1/security, oracle-content/trust, Aiken/compiler, generated-spend or native/ledger acceptance decision. Identify concrete ordinary fixes/limitations; revise, refuse or find evidence insufficient when warranted. No forced approval, no maximum string/reason lengths; substantive extra fields permitted.

## Ordinary requirements (complete)

```json
{
  "format": "ordinary-oracle-path-source-result-contract/1",
  "scope": "Existing CLI --oracle-results FILE argument forwarding and scoped actual test result only",
  "sourceRequirements": [
    {
      "id": "CLI-PATH-1",
      "requirement": "A supplied --oracle-results FILE reaches compileCustody as oracleResultsPath with its exact path text, including relative spelling and spaces."
    },
    {
      "id": "CLI-PATH-2",
      "requirement": "Absent --oracle-results passes undefined, preserving the existing function-parameter default; no historical default content or location is changed or followed."
    },
    {
      "id": "CLI-PATH-3",
      "requirement": "Help advertises the optional flag. Other supplied source/seed/cases/expected-release/expected-refund/residual/manifest/out options and existing result.ok-to-exit mapping remain unchanged."
    },
    {
      "id": "CLI-PATH-4",
      "requirement": "No compiler API, oracle validation, publication, Q1 owner/security test or security fixture behavior changes."
    }
  ],
  "resultRequirements": [
    {
      "id": "RESULT-1",
      "requirement": "Report exactly the pinned b988 source/tree and five actual selected Node argument-transport passes, zero failures/skips, exact stdout/stderr/elapsed receipts; no replacement run or broader suite claim."
    },
    {
      "id": "RESULT-2",
      "requirement": "The real CLI argv/dispatch and manifest read ran with direct operation imports replaced by capture probes; compileCustody itself, oracle contents, Aiken/SDK/native/ledger consumers did not run."
    },
    {
      "id": "RESULT-3",
      "requirement": "The capture-only compiler operation returns a non-success marker. Passing this test is path transport evidence, not compile success, output-path policy validation, oracle trust or generated-spending correspondence."
    },
    {
      "id": "RESULT-4",
      "requirement": "Cleanup receipts describe the owned root/session only. All original financial/compiler/native/target/Q1/product limits and gates remain unchanged."
    }
  ],
  "normativeLineage": [
    "Existing producer API signature202 and selected read312; exact excerpts and full-source hash provided",
    "Astra routine path/omission expectations are source-exposed corroboration, not a fresh provider audit or financial oracle",
    "EUC-012/EUC-013 remain whole existing requirements; this ordinary CLI handoff does not supply their publication/generated-spend acceptance"
  ],
  "priorSourceExposure": "Preparer authored the source proposal and has read actual implementations, tests and receipts. Packet preparation is not nonauthor approval."
}
```

## Exact existing producer API excerpt and pin

```json
{
  "fullSourcePin": {
    "commit": "b988764129e8828e3bc5b4d4b7660df61c3a184f",
    "path": "experiments/moriarty-eutxo/src/compile-custody.mjs",
    "gitBlob": "2857e14f74955cfad24891354992fcecc794d9e7",
    "sha256": "572f4ae5a767cadd47080eceb09ef56e920f507618a9fdacba27695851f41680",
    "bytes": 19587,
    "unchangedFromBase": true
  },
  "excerpts": [
    {
      "path": "experiments/moriarty-eutxo/src/compile-custody.mjs",
      "commit": "b988764129e8828e3bc5b4d4b7660df61c3a184f",
      "firstLine": 200,
      "lastLine": 205,
      "body": "}\n\nexport function compileCustody({ sourcePath, seedPath, casesPath, expectedReleasePath, expectedRefundPath, residualPath, manifest, outDir, oracleResultsPath = DEFAULT_ORACLE, aikenRun = runAiken, syntheticTime = null }) {\n  mkdirSync(outDir, { recursive: true });\n  const stale = clearArtifacts(outDir);\n  if (stale) return stale;\n"
    },
    {
      "path": "experiments/moriarty-eutxo/src/compile-custody.mjs",
      "commit": "b988764129e8828e3bc5b4d4b7660df61c3a184f",
      "firstLine": 310,
      "lastLine": 319,
      "body": "  let bound;\n  try {\n    bound = bindOracleSpends(JSON.parse(readFileSync(oracleResultsPath, 'utf8')), syntheticTime);\n  } catch (error) {\n    const failure = refuse(error.code || 'ORACLE_DIVERGENCE', error.message);\n    const drift = writeAdmission(outDir, failure, false);\n    if (drift) return drift;\n    return failure;\n  }\n  const validator = `${lowered.validator}\\n${bound.source}\\n`;\n"
    }
  ],
  "scope": "Existing option default and selected file read only; no oracle contents/trust or Q1 judgment."
}
```

## Full actual three-path source

### experiments/moriarty-eutxo/src/cli.mjs

Commit b988764129e8828e3bc5b4d4b7660df61c3a184f; Git blob 221ba947e71b7f5c5146eef53f32a0b16d7531fe; SHA256 42e2b8ecae7bddb80173d29d14d5951366fc4aebad4f7dbd86b738c2c18ad1a7.

```javascript
#!/usr/bin/env node
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { compilePublic } from './compile-public.mjs';
import { compileCustody } from './compile-custody.mjs';
import { validateDesigners } from './designers.mjs';
import { runNativeCaller } from './native-caller.mjs';
import { inspectLauncher, runNativeCase } from './native-consumer.mjs';
import { resolveOutputPath } from './native-runtime.mjs';

const here = dirname(fileURLToPath(import.meta.url));
const ROOT = resolve(here, '..');

function arg(flag, fallback) {
  const i = process.argv.indexOf(flag);
  if (i >= 0 && process.argv[i + 1] && !process.argv[i + 1].startsWith('--')) return process.argv[i + 1];
  return fallback;
}

function has(flag) {
  return process.argv.includes(flag);
}

function flagTrue(name) {
  if (!has(name)) return false;
  const value = arg(name, null);
  return value == null || value === 'true';
}

function integerArg(flag) {
  const text = arg(flag, null);
  if (text == null) return null;
  if (!/^(?:0|[1-9][0-9]*)$/.test(text)) return Number.NaN;
  const value = Number(text);
  return Number.isSafeInteger(value) ? value : Number.NaN;
}

function takeOut(fallback) {
  const output = resolveOutputPath(arg('--out', null), fallback);
  if (!output.ok) {
    process.stderr.write(`${output.code} ${output.path}\n`);
    process.exit(1);
  }
  return output.path;
}

function usage() {
  return `moriarty-eutxo public atomic experiment (not production, not private/PCD)

  node src/cli.mjs compile --source <file.mori> --manifest <manifest.json> [--out artifacts]
  node src/cli.mjs compile-custody --source <file.mori> --seed <seed.json> --manifest <manifest.json> [--oracle-results <file.json>] [--out t1-native-adapters-57/checks/cli-custody]
  node src/cli.mjs native [--out t1-native-adapters-57/checks/native-cli]
  node src/cli.mjs validate --designers <dir> --manifest <manifest.json> [--out artifacts]

Historical campaign check directories are refused.
Offline native does not exec a node server.
--execute with two agreeing authority files can spawn an owned process group.
The real cardano-node stays unarmed unless MORIARTY_ALLOW_REAL_NODE=1.

Pinned Aiken: /home/charl/.aiken/bin/aiken v1.1.21+42babe5 + stdlib v3.1.0
`;
}

const command = process.argv[2];
if (command === 'native') {
  const out = takeOut(join('/home/charl/moriarty-eutxo-validation-2026-10-12', 't1-native-adapters-57/checks/native-cli'));
  const integrated = Boolean(arg('--run-dir', null) || arg('--authority-a', null) || arg('--source', null) || has('--execute'));
  if (integrated) {
    const execute = flagTrue('--execute') || arg('--start-node', 'false') === 'true';
    const networkMagic = integerArg('--network-magic');
    const reviewA = arg('--authority-a', null);
    const authority = reviewA ? {
      reviewA,
      reviewB: arg('--authority-b', null),
      candidate: arg('--candidate', null),
    } : undefined;
    const runDir = arg('--run-dir', null);
    const nodeBin = arg('--node-bin', null);
    const request = {
      authority,
      sourcePath: arg('--source', null) || undefined,
      launch: {
        startNode: execute,
        nodeBin: nodeBin || undefined,
        runDir: runDir || undefined,
        configDir: arg('--config-dir', null) || undefined,
        port: arg('--port', null) ? Number(arg('--port', null)) : undefined,
        socketPath: arg('--socket', null) || undefined,
      },
      magic: networkMagic,
      networkMagic,
    };
    const cashSourcePath = arg('--cash-source', null);
    if (cashSourcePath) request.cashSource = readFileSync(cashSourcePath, 'utf8');
    const voteA = arg('--cash-vote-a', null);
    if (voteA) {
      request.cashVotes = { reviewA: voteA, reviewB: arg('--cash-vote-b', null), candidate: arg('--cash-candidate', null) };
    }
    const designA = arg('--cash-design-a', null);
    if (designA) {
      request.cashDesign = {
        reviewA: designA,
        reviewB: arg('--cash-design-b', null),
        candidate: arg('--cash-design-candidate', null),
        sourcePath: arg('--cash-design-source', null),
      };
    }
    const protocolCli = arg('--protocol-cli', null);
    if (protocolCli) {
      request.protocol = {
        cliPath: protocolCli,
        socket: arg('--protocol-socket', null),
        execute: true,
        magic: networkMagic,
      };
    }
    if (arg('--tx-cost-memory', null) && arg('--tx-cost-steps', null)) {
      request.cost = { memory: arg('--tx-cost-memory', null), steps: arg('--tx-cost-steps', null) };
      if (arg('--tx-bytes', null)) request.txBytes = Number(arg('--tx-bytes', null));
    }
    const timeCli = arg('--time-cli', null);
    if (timeCli) request.time = { cliPath: timeCli, socket: arg('--time-socket', null), execute: true, magic: networkMagic };
    const siblingBin = arg('--sibling-compiler', null);
    if (siblingBin) {
      request.sibling = {
        compilerBin: siblingBin,
        sourcePath: arg('--sibling-source', null),
        previousProgramHash: arg('--previous-program-hash', null),
      };
    }
    const keyCli = arg('--key-cli', null);
    if (keyCli && runDir) request.keys = { cliPath: keyCli, runDir, magic: networkMagic };
    const utxoCli = arg('--utxo-cli', null);
    if (utxoCli && runDir) {
      request.utxoQuery = { cliPath: utxoCli, socket: arg('--utxo-socket', null), runDir, execute: true, magic: networkMagic };
    }
    if (flagTrue('--build-body')) {
      request.buildBody = true;
      request.financialBody = true;
      request.requiredSignerHash = arg('--required-signer-hash', null);
      request.cliPath = arg('--body-cli', null) || undefined;
    }
    const submitBin = arg('--submit-bin', null);
    if (submitBin) {
      request.race = {
        submitBin,
        pollBin: arg('--poll-bin', null),
        signBin: arg('--sign-bin', null),
        logPath: arg('--race-log', null),
        leftActor: arg('--left-actor', 'release'),
        rightActor: arg('--right-actor', 'cancel'),
        leftResource: arg('--left-resource', 'release-resource'),
        rightResource: arg('--right-resource', 'cancel-resource'),
      };
    }
    const readbackCli = arg('--readback-cli', null);
    if (readbackCli) request.readback = { cliPath: readbackCli, socket: arg('--readback-socket', null), magic: networkMagic };
    if (arg('--receipt-status', null)) request.receiptStatus = arg('--receipt-status', null);
    const faucetBin = arg('--faucet-bin', null);
    if (faucetBin) request.faucet = { bin: faucetBin, logPath: arg('--faucet-log', null) };
    const ran = await runNativeCaller(request);
    const stageCodes = {};
    for (const [name, stage] of Object.entries(ran.stages || {})) stageCodes[name] = stage?.code || null;
    const witnessStage = ran.stages?.witness;
    const report = {
      ok: ran.ok === true,
      code: ran.code,
      ledgerExecution: false,
      nativePins: ran.nativePins === true,
      ledgerSubmitted: false,
      confirmed: ran.confirmed === true,
      submitted: ran.submitted === true,
      startedNode: ran.startedNode === true,
      shellExitMeansLedger: false,
      unitOnly: ran.unitOnly === true,
      ceilingRemoved: false,
      source: ran.stages?.source || null,
      witness: witnessStage ? {
        code: witnessStage.code || null,
        unitOnly: witnessStage.unitOnly === true,
        forced: witnessStage.forced === true,
        provenance: witnessStage.provenance || 'unit-io',
      } : {
        code: null,
        unitOnly: ran.unitOnly === true,
        forced: false,
        provenance: 'not-constructed',
      },
      authority: {
        code: ran.stages?.authority?.code || null,
        requiredManifestDigest: ran.stages?.authority?.requiredManifestDigest ?? null,
        agreed: ran.stages?.authority?.code === 'AUTHORITY_AGREED',
      },
      launcher: {
        code: ran.stages?.launcher?.code || null,
        started: ran.stages?.launcher?.started === true,
        spawnAttempted: ran.stages?.launcher?.spawnAttempted === true,
        setsid: ran.stages?.launcher?.setsid === true,
        binaryPresent: ran.stages?.launcher?.binaryPresent === true,
        portOk: ran.stages?.launcher?.port?.ok === true,
      },
      stages: stageCodes,
      cashDesign: {
        code: ran.stages?.cashDesign?.code || null,
        designAdopted: ran.stages?.cashDesign?.designAdopted === true,
        cashCanMint: false,
        ledgerFunding: false,
        reviewClass: ran.stages?.cashDesign?.reviewClass || null,
      },
      mint: {
        code: ran.stages?.mint?.code || null,
        cashPolicyId: ran.stages?.mint?.cashPolicyId || null,
        compiledScript: ran.stages?.mint?.compiledScript === true,
        markerNotNativeFunding: ran.stages?.mint?.markerNotNativeFunding === true,
      },
      headOpen: {
        code: ran.stages?.body?.code || null,
        headMintsCash: ran.stages?.body?.headMintsCash === true,
        cashSeedReused: ran.stages?.body?.cashSeedReused === true,
        headPolicyId: ran.stages?.body?.headPolicyId || null,
        holding: ran.stages?.body?.holding || null,
        instanceId: ran.stages?.body?.instanceId || null,
      },
      out,
    };
    mkdirSync(out, { recursive: true });
    const text = JSON.stringify(report, null, 2) + '\n';
    writeFileSync(join(out, 'native-report.json'), text);
    process.stdout.write(text);
    process.exit(0);
  }
  const startNode = arg('--start-node', 'false') === 'true';
  const launcherRequest = { startNode };
  const nodeBin = arg('--node-bin', null);
  const configDir = arg('--config-dir', null);
  const portArg = arg('--port', null);
  if (nodeBin) launcherRequest.nodeBin = nodeBin;
  if (configDir) launcherRequest.configDir = configDir;
  if (portArg) launcherRequest.port = Number(portArg);
  const key = arg('--signing-key', null);
  const applied = arg('--applied', null);
  const reportPath = arg('--compiled-report', null);
  const scriptHash = arg('--script-hash', null);
  const cashPolicy = arg('--cash-policy', null);
  const socket = arg('--socket', null);
  const bodyHex = arg('--tx-body-hex', null);
  const request = { launcher: launcherRequest };
  if (key || applied || reportPath || scriptHash || cashPolicy) {
    request.pins = {
      keyPath: key,
      appliedBlueprintPath: applied,
      compiledReportPath: reportPath,
      scriptHash: scriptHash || '',
      cashPolicyId: cashPolicy || '',
      utxoQuery: socket ? { queried: false, socket, unitOnly: false } : undefined,
    };
  }
  if (key || bodyHex) {
    request.witness = {
      signingKeyPath: key,
      txBodyCborHex: bodyHex,
      txBodyType: arg('--tx-body-type', 'Tx ConwayEra'),
    };
  }
  if (socket) request.budget = { queried: false };
  const launcher = await inspectLauncher(launcherRequest);
  const ran = await runNativeCase(request);
  const report = {
    ok: false,
    code: ran.code,
    ledgerExecution: false,
    nativePins: false,
    confirmed: false,
    submitted: false,
    startedNode: launcher.started === true,
    shellExitMeansLedger: false,
    launcher: {
      code: launcher.code,
      started: launcher.started === true,
      setsid: launcher.setsid === true,
      binaryPresent: launcher.binaryPresent === true,
      portOk: launcher.port?.ok === true,
    },
    stages: {
      pins: ran.stages?.pins?.code || null,
      witness: ran.stages?.witness?.code || null,
      budget: ran.stages?.budget?.code || null,
      submit: ran.stages?.submit?.code || null,
    },
    out,
  };
  mkdirSync(out, { recursive: true });
  const text = JSON.stringify(report, null, 2) + '\n';
  writeFileSync(join(out, 'native-report.json'), text);
  process.stdout.write(text);
  process.exit(0);
}
if (command === 'compile-custody') {
  const task = '/home/charl/moriarty-eutxo-validation-2026-10-12';
  const designer = join(task, 'designers/02-escrow-time');
  const manifestPath = arg('--manifest', join(ROOT, 'examples/custody-unit.manifest.json'));
  const manifest = JSON.parse(readFileSync(manifestPath, 'utf8'));
  const source = arg('--source', join(designer, 'escrow-custody-v5.mori'));
  const seed = arg('--seed', join(designer, 'financial-seed-custody-v5.json'));
  const out = takeOut(join(task, 't1-native-adapters-57/checks/cli-custody'));
  const result = compileCustody({
    sourcePath: source,
    seedPath: seed,
    casesPath: arg('--cases', join(designer, 'cases.json')),
    expectedReleasePath: arg('--expected-release', join(designer, 'E13-release-real-custody.expected.json')),
    expectedRefundPath: arg('--expected-refund', join(designer, 'E11-refund-real-custody.expected.json')),
    residualPath: arg('--residual', join(designer, 'financial-residual-custody-v5.json')),
    oracleResultsPath: arg('--oracle-results', undefined),
    manifest,
    outDir: out,
  });
  process.stdout.write(JSON.stringify({
    ok: result.ok,
    code: result.code,
    ledgerExecution: false,
    nativePins: false,
    layer: result.layer || 'source-uplc-unit',
    hashes: result.hashes,
    instanceId: result.instanceId,
    checkStatus: result.check?.status,
    buildStatus: result.build?.status,
    blueprint: result.blueprintPath,
    validator: result.validatorPath,
    missing: result.check?.missing,
  }, null, 2) + '\n');
  process.exit(result.ok ? 0 : 1);
}
if (command !== 'compile' && command !== 'validate') {
  process.stderr.write(usage());
  process.exit(2);
}

const manifestPath = arg('--manifest', join(ROOT, 'examples/revision-step.manifest.json'));
const manifest = JSON.parse(readFileSync(manifestPath, 'utf8'));
const out = takeOut(join(ROOT, 'artifacts'));

if (command === 'compile') {
  const source = arg('--source', join(ROOT, 'examples/revision-step.mori'));
  const result = compilePublic({ source, manifest, outDir: out });
  process.stdout.write(JSON.stringify({
    ok: result.ok,
    programHash: result.metadata?.programHash,
    admission: result.admission?.ok === false ? result.admission : { ok: true, capability: result.admission?.capability, privateVerification: result.admission?.privateVerification },
    checkStatus: result.check?.status,
    buildStatus: result.build?.status,
    artifacts: result.artifacts,
    blueprint: result.blueprintPath,
    validator: result.validatorPath,
    checkStderr: result.ok ? undefined : result.check?.stderr,
    buildStderr: result.ok ? undefined : result.build?.stderr,
  }, null, 2) + '\n');
  process.exit(result.ok ? 0 : 1);
}

const designers = arg('--designers');
if (!designers) {
  process.stderr.write('validate requires explicit --designers path\n' + usage());
  process.exit(2);
}
const matrix = validateDesigners({ designers: resolve(designers), manifest, outDir: out });
const ok = matrix.ok === true;
process.stdout.write(JSON.stringify({
  ok,
  rows: matrix.rows.length,
  patternCount: matrix.patternCount,
  verifiers: matrix.verifiers?.map(v => ({ designer: v.designer, exit_code: v.exit_code, ok: v.ok })),
  compiledExample: matrix.compiledExample,
  discrepancies: matrix.discrepancies,
  missingPatterns: matrix.missingPatterns,
  artifacts: join(out, 'matrix.json'),
  capability: matrix.capability,
  production: false,
}, null, 2) + '\n');
process.exit(ok ? 0 : 1);
```

### experiments/moriarty-eutxo/tests/compile-custody-cli-args.test.mjs

Commit b988764129e8828e3bc5b4d4b7660df61c3a184f; Git blob 9745c278f0137778bf8e369a977f4f313f2a91a5; SHA256 a398ddb39bec8f5d9d34cdbbe8c6f0316226eb0523c5b47e2c598d51f205c7c7.

```javascript
// Public CLI argument transport only; no compiler, oracle or native operation.
import test from 'node:test';
import assert from 'node:assert/strict';
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const cli = fileURLToPath(new URL('../src/cli.mjs', import.meta.url));
const captureHook = fileURLToPath(new URL('./support/compile-custody-cli-capture.mjs', import.meta.url));

function captureOptions(oracleResultsPath) {
  const dir = mkdtempSync(join(tmpdir(), 'mori-custody-cli-args-'));
  try {
    const manifest = { purpose: 'CLI argument transport only' };
    const manifestPath = join(dir, 'manifest.json');
    const capturePath = join(dir, 'captured-options.json');
    writeFileSync(manifestPath, JSON.stringify(manifest));
    const inputs = {
      sourcePath: 'inputs/source.mori',
      seedPath: 'inputs/seed.json',
      casesPath: 'inputs/cases.json',
      expectedReleasePath: 'inputs/release.json',
      expectedRefundPath: 'inputs/refund.json',
      residualPath: 'inputs/residual.json',
      manifest,
      outDir: join(dir, 'output artifacts'),
    };
    const args = [
      '--import', captureHook, cli, 'compile-custody',
      '--source', inputs.sourcePath,
      '--seed', inputs.seedPath,
      '--cases', inputs.casesPath,
      '--expected-release', inputs.expectedReleasePath,
      '--expected-refund', inputs.expectedRefundPath,
      '--residual', inputs.residualPath,
      '--manifest', manifestPath,
      '--out', inputs.outDir,
    ];
    if (oracleResultsPath !== undefined) args.push('--oracle-results', oracleResultsPath);
    const child = spawnSync(process.execPath, args, {
      cwd: dir,
      encoding: 'utf8',
      env: { ...process.env, MORI_CLI_ARGUMENT_CAPTURE: capturePath },
    });
    assert.equal(child.error, undefined);
    assert.equal(child.status, 1, child.stderr);
    assert.equal(child.stderr, '');
    // The operation probe deliberately returns no compile-success result.
    assert.equal(JSON.parse(child.stdout).code, 'CLI_ARGUMENT_MAPPING_CAPTURE');
    const captured = JSON.parse(readFileSync(capturePath, 'utf8'));
    const expected = oracleResultsPath === undefined
      ? inputs
      : { ...inputs, oracleResultsPath };
    assert.deepEqual(captured.options, expected);
    assert.equal(captured.oracleResultsPathType,
      oracleResultsPath === undefined ? 'undefined' : 'string');
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
}

for (const path of ['fixtures/public/oracle-a.json', 'fixtures/public/oracle-b.json']) {
  test(`compile-custody forwards the exact oracle path ${path}`, () => {
    captureOptions(path);
  });
}

test('compile-custody forwards an explicit oracle results path with spaces', () => {
  captureOptions(join(tmpdir(), 'independent oracle directory', 'RESULTS.json'));
});

test('compile-custody preserves a relative oracle results path for its existing API', () => {
  captureOptions('independent oracle directory/RESULTS.json');
});

test('compile-custody omission leaves the existing API default selected', () => {
  captureOptions(undefined);
});
```

### experiments/moriarty-eutxo/tests/support/compile-custody-cli-capture.mjs

Commit b988764129e8828e3bc5b4d4b7660df61c3a184f; Git blob 44638ec10739d7e120c963b6e9be1f041864bf3c; SHA256 909977e18aa17c0910544398738dabc74369a7731e6f6200380be545e85c11be.

```javascript
// Capture only the existing compileCustody options boundary of the real CLI.
import { registerHooks } from 'node:module';

const cliUrl = new URL('../../src/cli.mjs', import.meta.url).href;
const unused = (names) => names.map((name) =>
  `export function ${name}() { throw new Error('unused CLI operation: ${name}'); }`
).join('\n');
const probes = new Map([
  ['./compile-public.mjs', unused(['compilePublic'])],
  ['./compile-custody.mjs', `
    import { writeFileSync } from 'node:fs';
    export function compileCustody(options) {
      writeFileSync(process.env.MORI_CLI_ARGUMENT_CAPTURE, JSON.stringify({
        options,
        oracleResultsPathType: typeof options.oracleResultsPath,
      }));
      return { ok: false, code: 'CLI_ARGUMENT_MAPPING_CAPTURE', layer: 'argument-mapping-only' };
    }
  `],
  ['./designers.mjs', unused(['validateDesigners'])],
  ['./native-caller.mjs', unused(['runNativeCaller'])],
  ['./native-consumer.mjs', unused(['inspectLauncher', 'runNativeCase'])],
  ['./native-runtime.mjs', `
    export function resolveOutputPath(path, fallback) {
      return { ok: true, path: path ?? fallback };
    }
  `],
]);

registerHooks({
  resolve(specifier, context, nextResolve) {
    if (context.parentURL === cliUrl && probes.has(specifier)) {
      return {
        url: `data:text/javascript,${encodeURIComponent(probes.get(specifier))}`,
        shortCircuit: true,
      };
    }
    return nextResolve(specifier, context);
  },
});
```

## Complete actual check plan and terminal receipt

### CHECK-PLAN.json

```json
{
  "recordedAt": "2026-10-18T14:09:43.885721+00:00",
  "head": "b988764129e8828e3bc5b4d4b7660df61c3a184f",
  "tree": "32a27d88760401c65be4606527274a15410792ea",
  "argv": [
    "/usr/bin/timeout",
    "--signal=TERM",
    "--kill-after=5s",
    "30s",
    "/home/charl/.foreman/tools/fnm/node-versions/v24.21.0/installation/bin/node",
    "--test",
    "/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018/experiments/moriarty-eutxo/tests/compile-custody-cli-args.test.mjs"
  ],
  "environment": {
    "PATH": "/usr/bin:/bin",
    "LANG": "C.UTF-8",
    "TMPDIR": "/tmp/doctorwatson-custody-cli-args-cqi6sgh2/tmp"
  },
  "sourcePins": [
    {
      "path": "experiments/moriarty-eutxo/src/cli.mjs",
      "sha256": "42e2b8ecae7bddb80173d29d14d5951366fc4aebad4f7dbd86b738c2c18ad1a7"
    },
    {
      "path": "experiments/moriarty-eutxo/tests/compile-custody-cli-args.test.mjs",
      "sha256": "a398ddb39bec8f5d9d34cdbbe8c6f0316226eb0523c5b47e2c598d51f205c7c7"
    },
    {
      "path": "experiments/moriarty-eutxo/tests/support/compile-custody-cli-capture.mjs",
      "sha256": "909977e18aa17c0910544398738dabc74369a7731e6f6200380be545e85c11be"
    }
  ],
  "nodeSha256": "7fde7b8afa198da66257f42ee2001d874c7355631e6d1579a5fb5ef1f246df4c",
  "oracleCompilerSourceUntouched": true,
  "scope": "Actual CLI argv dispatch with only direct operation imports replaced by argument capture probes. No producer/compiler/Aiken/oracle/SDK/native/ledger calls; no compile-success result claimed.",
  "ownedRoot": "/tmp/doctorwatson-custody-cli-args-cqi6sgh2"
}
```

### RESULT.json

```json
{
  "finishedAt": "2026-10-18T14:09:44.216537+00:00",
  "exitCode": 0,
  "wallSeconds": 0.23836761200072942,
  "head": "b988764129e8828e3bc5b4d4b7660df61c3a184f",
  "tree": "32a27d88760401c65be4606527274a15410792ea",
  "stdoutSha256": "38c9668d2ed81f296f55ad8501a805c4986965ea0b6cd4e87795d442432d1feb",
  "stderrSha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
  "ownedSessionRemaining": [],
  "sourcePinsUnchanged": true,
  "workingTreeClean": true
}
```

### CHILD.json

```json
{
  "pid": 800143,
  "session": 800143
}
```

### CLEANUP.json

```json
{
  "recordedAt": "2026-10-18T14:09:44.273178+00:00",
  "ownedRoot": "/tmp/doctorwatson-custody-cli-args-cqi6sgh2",
  "existsAfter": false
}
```

## Complete actual stdout

```text
✔ compile-custody forwards the exact oracle path fixtures/public/oracle-a.json (29.645006ms)
✔ compile-custody forwards the exact oracle path fixtures/public/oracle-b.json (29.414341ms)
✔ compile-custody forwards an explicit oracle results path with spaces (29.274841ms)
✔ compile-custody preserves a relative oracle results path for its existing API (30.543776ms)
✔ compile-custody omission leaves the existing API default selected (30.820563ms)
ℹ tests 5
ℹ suites 0
ℹ pass 5
ℹ fail 0
ℹ cancelled 0
ℹ skipped 0
ℹ todo 0
ℹ duration_ms 202.867133
```

Actual stderr is exactly zero bytes (SHA256 e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855). Five selected test names passed, none failed or skipped. The outer wall receipt is 0.23836761200072942 seconds; Node reported test duration202.867133ms. These are different recorded measures, not substituted timings. Root owned-root cleanup receipt says existsAfter=false; this does not establish unrelated-process or syscall exclusion.

The child tests ran the actual cli.mjs parser/dispatch and read their own manifest. Their direct operation-import probes prevented importing/executing compiler/native consumers. The non-success probe captured argument objects, and tests asserted exact custom-path forwarding/omission, all supplied options and the existing false-result exit mapping. Omitted/default behavior means undefined reached the call boundary plus the unchanged JavaScript API default contract; no historical oracle file or actual compiler default was opened. Output-path policy was probed, so its policy is outside these results. No generated spend, proof, SDK/native or real oracle-content evidence follows.

All seven original public execution receipts are physical under actual-receipts/, unchanged. ASTRA-EXPECTATIONS.json carries the complete source-exposed routine expectations; full base CLI and actual three-path patch are physical. CURRENT-SOURCE-PINS ties each actual test input to the root execution plan and immutable Git blob. No private/provider log, credential, external excluded-tree content or financial oracle payload is copied. Historical absolute path strings in the full public CLI and exact root receipts are inert source/evidence, not traversed.
