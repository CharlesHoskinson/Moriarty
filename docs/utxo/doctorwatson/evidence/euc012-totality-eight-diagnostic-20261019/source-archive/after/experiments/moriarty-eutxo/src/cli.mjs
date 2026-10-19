#!/usr/bin/env node
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { compilePublic } from './compile-public.mjs';
import { compileCustody, verifyCustodyPublication } from './compile-custody.mjs';
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
  node src/cli.mjs compile-custody --source <file.mori> --seed <seed.json> --manifest <manifest.json> [--out t1-native-adapters-57/checks/cli-custody]
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
    manifest,
    outDir: out,
  });
  const publication = result.ok ? verifyCustodyPublication({ outDir: out }) : null;
  const consumedOk = result.ok === true && publication?.ok === true;
  process.stdout.write(JSON.stringify({
    ok: consumedOk,
    code: consumedOk ? result.code : (publication?.code || result.code),
    ledgerExecution: false,
    nativePins: false,
    layer: result.layer || 'source-uplc-unit',
    hashes: result.hashes,
    instanceId: result.instanceId,
    checkStatus: result.check?.status,
    buildStatus: result.build?.status,
    blueprint: consumedOk ? result.blueprintPath : null,
    publication: consumedOk ? (result.publicationPath || null) : null,
    publicationVerified: consumedOk,
    validator: result.validatorPath,
    missing: result.check?.missing,
  }, null, 2) + '\n');
  process.exit(consumedOk ? 0 : 1);
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
